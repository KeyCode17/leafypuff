package com.leafypuff.data

import android.content.Context
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Base64
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

private const val KeystoreProvider = "AndroidKeyStore"
private const val KeyAlias = "leafypuff.device.key"
private const val Transformation = "AES/GCM/NoPadding"
private const val PreferencesName = "leafypuff.device"
private const val CiphertextKey = "passphrase.ciphertext"
private const val NonceKey = "passphrase.nonce"
private const val TagBits = 128
private const val DeviceKeyBytes = 32

class DeviceKey(private val context: Context) {
    fun bytes(): ByteArray {
        val preferences = context.getSharedPreferences(PreferencesName, Context.MODE_PRIVATE)
        val stored = preferences.getString(CiphertextKey, null)
        val nonce = preferences.getString(NonceKey, null)
        if (stored != null && nonce != null) {
            return open(decode(stored), decode(nonce))
        }

        val fresh = ByteArray(DeviceKeyBytes).also { java.security.SecureRandom().nextBytes(it) }
        val cipher = Cipher.getInstance(Transformation).apply { init(Cipher.ENCRYPT_MODE, key()) }
        val sealed = cipher.doFinal(fresh)
        preferences.edit()
            .putString(CiphertextKey, encode(sealed))
            .putString(NonceKey, encode(cipher.iv))
            .apply()
        return fresh
    }

    fun forget() {
        context.getSharedPreferences(PreferencesName, Context.MODE_PRIVATE)
            .edit()
            .clear()
            .apply()
    }

    private fun open(sealed: ByteArray, nonce: ByteArray): ByteArray {
        val cipher = Cipher.getInstance(Transformation).apply {
            init(Cipher.DECRYPT_MODE, key(), GCMParameterSpec(TagBits, nonce))
        }
        return cipher.doFinal(sealed)
    }

    private fun key(): SecretKey {
        val store = KeyStore.getInstance(KeystoreProvider).apply { load(null) }
        val existing = store.getEntry(KeyAlias, null) as? KeyStore.SecretKeyEntry
        if (existing != null) {
            return existing.secretKey
        }
        val generator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, KeystoreProvider)
        generator.init(
            KeyGenParameterSpec.Builder(
                KeyAlias,
                KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT,
            )
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .build(),
        )
        return generator.generateKey()
    }

    private fun encode(bytes: ByteArray): String = Base64.encodeToString(bytes, Base64.NO_WRAP)

    private fun decode(text: String): ByteArray = Base64.decode(text, Base64.NO_WRAP)
}
