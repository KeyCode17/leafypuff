package com.leafypuff.data

import kotlin.test.Test
import kotlin.test.assertEquals

class VaultRouteTest {
    @Test
    fun aVaultHeldOnlyOnThisDeviceIsReopenedRatherThanCreatedOver() {
        assertEquals(VaultRoute.Reopen, vaultRoute(restored = false, present = true))
    }

    @Test
    fun aVaultTheServerHandedBackIsRestored() {
        assertEquals(VaultRoute.Restore, vaultRoute(restored = true, present = false))
        assertEquals(VaultRoute.Restore, vaultRoute(restored = true, present = true))
    }

    @Test
    fun aVaultThatExistsNowhereIsCreated() {
        assertEquals(VaultRoute.Create, vaultRoute(restored = false, present = false))
    }
}
