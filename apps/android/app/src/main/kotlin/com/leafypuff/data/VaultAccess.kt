package com.leafypuff.data

import com.leafypuff.core.CoreClient
import kotlinx.datetime.Clock

sealed interface VaultOpened {
    data object Restored : VaultOpened

    data class Created(val recoveryCode: String) : VaultOpened
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
    ): VaultOpened {
        val restored = client.restoreVault(apiBaseUrl, accessToken)
        val outcome = when {
            restored -> {
                client.unlock(password)
                VaultOpened.Restored
            }

            else -> {
                val code = client.createVault(password)
                client.uploadVault(apiBaseUrl, accessToken, Clock.System.now().toEpochMilliseconds())
                VaultOpened.Created(code)
            }
        }
        client.rememberOnDevice(deviceKey.bytes())
        return outcome
    }

    suspend fun signOut() {
        client.forgetDeviceKey()
        client.lock()
        deviceKey.forget()
    }
}
