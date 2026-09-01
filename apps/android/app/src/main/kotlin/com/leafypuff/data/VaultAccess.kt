package com.leafypuff.data

import com.leafypuff.core.CoreClient
import kotlinx.datetime.Clock

sealed interface VaultOpened {
    data object Restored : VaultOpened

    data object Reopened : VaultOpened

    data class Created(val recoveryCode: String) : VaultOpened
}

enum class VaultRoute {
    Restore,
    Reopen,
    Create,
}

internal fun vaultRoute(restored: Boolean, present: Boolean): VaultRoute = when {
    restored -> VaultRoute.Restore
    present -> VaultRoute.Reopen
    else -> VaultRoute.Create
}

enum class VaultKeep {
    OnThisDevice,
    OnTheServer,
}

class VaultAccess(private val client: CoreClient, private val deviceKey: DeviceKey) {
    suspend fun openWithDeviceKey(): Boolean {
        if (!client.hasDeviceSlot()) {
            return false
        }
        return runCatching { client.unlockWithDeviceKey(deviceKey.bytes()) }.isSuccess
    }

    suspend fun openWith(
        apiBaseUrl: String,
        accessToken: String,
        password: String,
        onKeepFailed: (VaultKeep, Throwable) -> Unit,
    ): VaultOpened {
        val restored = client.restoreVault(apiBaseUrl, accessToken)
        val outcome = when (vaultRoute(restored = restored, present = client.hasVault())) {
            VaultRoute.Restore -> {
                client.unlock(password)
                VaultOpened.Restored
            }

            VaultRoute.Reopen -> {
                client.unlock(password)
                VaultOpened.Reopened
            }

            VaultRoute.Create -> VaultOpened.Created(client.createVault(password))
        }

        runCatching { client.rememberOnDevice(deviceKey.bytes()) }
            .onFailure { failure -> onKeepFailed(VaultKeep.OnThisDevice, failure) }
        runCatching {
            client.uploadVault(apiBaseUrl, accessToken, Clock.System.now().toEpochMilliseconds())
        }.onFailure { failure -> onKeepFailed(VaultKeep.OnTheServer, failure) }

        return outcome
    }

    suspend fun resealWithRecoveryCode(
        apiBaseUrl: String,
        accessToken: String,
        code: String,
        password: String,
        onKeepFailed: (VaultKeep, Throwable) -> Unit,
    ) {
        client.resealWithRecoveryCode(
            baseUrl = apiBaseUrl,
            accessToken = accessToken,
            code = code,
            passphrase = password,
            updatedAtMs = Clock.System.now().toEpochMilliseconds(),
        )
        runCatching { client.rememberOnDevice(deviceKey.bytes()) }
            .onFailure { failure -> onKeepFailed(VaultKeep.OnThisDevice, failure) }
    }

    suspend fun signOut() {
        client.forgetDeviceKey()
        client.lock()
        deviceKey.forget()
    }
}
