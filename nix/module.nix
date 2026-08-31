self:
{ config, lib, pkgs, ... }:
let
  cfg = config.services.leafypuff;
in
{
  options.services.leafypuff = {
    enable = lib.mkEnableOption "leafyPuff sync API";

    domain = lib.mkOption {
      type = lib.types.str;
      description = "Public hostname served over TLS.";
    };

    acmeEmail = lib.mkOption {
      type = lib.types.str;
      description = "Contact address for Let's Encrypt.";
    };

    environmentFile = lib.mkOption {
      type = lib.types.path;
      description = "Holds DATABASE_URL, MINIO_* and RESEND_API_KEY. Never enters the nix store.";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 8080;
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.leafypuff = {
      isSystemUser = true;
      group = "leafypuff";
    };
    users.groups.leafypuff = { };

    services.postgresql = {
      enable = true;
      ensureDatabases = [ "leafypuff" ];
      ensureUsers = [
        {
          name = "leafypuff";
          ensureDBOwnership = true;
        }
      ];
    };

    # Loopback on purpose: every object read and write goes through the API, so MinIO
    # gets no vhost and no firewall hole. Exposing it would be a second public surface.
    services.minio = {
      enable = true;
      listenAddress = "127.0.0.1:9000";
      consoleAddress = "127.0.0.1:9001";
      rootCredentialsFile = cfg.environmentFile;
    };

    systemd.services.leafypuff-api = {
      description = "leafyPuff sync API";
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" "postgresql.service" "minio.service" ];
      wants = [ "network-online.target" ];
      serviceConfig = {
        ExecStart = "${self.packages.${pkgs.stdenv.hostPlatform.system}.leafypuff-api}/bin/leafypuff-api";
        EnvironmentFile = cfg.environmentFile;
        Environment = [ "PORT=${toString cfg.port}" ];
        User = "leafypuff";
        Group = "leafypuff";
        Restart = "on-failure";
        RestartSec = 5;
        NoNewPrivileges = true;
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
      };
    };

    # mkDefault: web-porto and wa-bot on this host may already define the ACME contact,
    # and two definitions of a str option conflict even when the values are identical.
    security.acme.acceptTerms = true;
    security.acme.defaults.email = lib.mkDefault cfg.acmeEmail;

    services.nginx = {
      enable = true;
      recommendedProxySettings = true;
      recommendedTlsSettings = true;
      virtualHosts.${cfg.domain} = {
        forceSSL = true;
        enableACME = true;
        locations."/".proxyPass = "http://127.0.0.1:${toString cfg.port}";
      };
    };

    networking.firewall.allowedTCPPorts = [ 80 443 ];
  };
}
