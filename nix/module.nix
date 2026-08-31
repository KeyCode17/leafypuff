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

    dnsProvider = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = ''
        lego DNS-01 provider. Required when the zone sits behind a proxy such as
        Cloudflare, because an HTTP-01 challenge never reaches nginx. Null keeps
        the webroot challenge.
      '';
    };

    acmeEnvironmentFile = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      description = "Holds the DNS provider credential lego reads. Required with dnsProvider.";
    };

    storageListen = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1:3900";
      description = "Where the S3 API listens. Loopback only; the API is its sole reader.";
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

    # Loopback on purpose: every object read and write goes through the API, so the
    # object store gets no vhost and no firewall hole. Exposing it would be a second
    # public surface.
    #
    # Garage rather than MinIO: nixpkgs marks minio insecure with six unfixed CVEs,
    # two of them unauthenticated object write, and upstream has abandoned it.
    services.garage = {
      enable = true;
      package = pkgs.garage;
      environmentFile = cfg.environmentFile;
      settings = {
        replication_factor = 1;
        db_engine = "lmdb";
        metadata_dir = "/var/lib/garage/meta";
        data_dir = "/var/lib/garage/data";
        rpc_bind_addr = "127.0.0.1:3901";
        s3_api = {
          api_bind_addr = cfg.storageListen;
          s3_region = "garage";
        };
      };
    };

    systemd.services.leafypuff-api = {
      description = "leafyPuff sync API";
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" "postgresql.service" "garage.service" ];
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
        enableACME = cfg.dnsProvider == null;
        useACMEHost = if cfg.dnsProvider == null then null else cfg.domain;
        locations."/".proxyPass = "http://127.0.0.1:${toString cfg.port}";
      };
    };

    security.acme.certs = lib.mkIf (cfg.dnsProvider != null) {
      ${cfg.domain} = {
        inherit (cfg) dnsProvider;
        environmentFile = cfg.acmeEnvironmentFile;
        group = "nginx";
      };
    };

    networking.firewall.allowedTCPPorts = [ 80 443 ];
  };
}
