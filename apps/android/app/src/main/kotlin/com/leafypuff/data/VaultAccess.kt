package com.leafypuff.data

import com.leafypuff.core.CoreClient
import kotlinx.datetime.Clock

/**
 * What happened when the vault was opened. A fresh vault hands back a recovery code, and that is
 * the only moment it exists: nothing writes it down.
 */
sealed interface VaultOpened {
    data object Restored : VaultOpened

    data class Created(val recoveryCode: String) : VaultOpened
}

/**
 * Opening the diary's vault. The account password is the key on a device that has not seen this
 * account before; after that the handset keeps its own wrapped copy so it stops asking.
 */
class VaultAccess(private val client: CoreClient, private val deviceKey: DeviceKey) {

    /// True when this handset already holds a copy it can open on its own.
    suspend fun openWithDeviceKey(): Boolean {
        if (!client.hasDeviceSlot()) {
            return false
        }
        return runCatching { client.unlockWithDeviceKey(deviceKey.bytes()) }.isSuccess
    }

    /**
     * Called with the password the owner just typed. The account's vault is fetched if it has one
     * and created if it does not, and either way this handset keeps a copy afterwards so the next
     * launch is silent.
     */
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

    /**
     * Signing out drops this handset's copy of the key and the key that wrapped it. The diary
     * stays on disk, sealed; the account password is the only way back in, which is what makes
     * signing out on a shared phone mean something.
     */
    suspend fun signOut() {
        client.forgetDeviceKey()
        client.lock()
        deviceKey.forget()
    }
}
