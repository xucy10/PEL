package com.movtery.zalithlauncher.bridge

import com.sun.jna.Library
import com.sun.jna.Native

/**
 * Rust JNI Bridge for ZalithLauncher
 *
 * This class provides the interface to the Rust native library
 * for performance-critical operations like file I/O, SHA1 hashing,
 * and JVM launching.
 */
object RustBridge {

    private const val LIB_NAME = "zalith_jni"

    // Load the native library
    init {
        try {
            System.loadLibrary(LIB_NAME)
        } catch (e: UnsatisfiedLinkError) {
            // Try to find library in app's library path
            val libPath = System.getProperty("java.library.path")
            e.printStackTrace()
        }
    }

    /**
     * Initialize the Rust library
     * @return 0 on success, -1 on failure
     */
    @JvmStatic
    external fun rustInit(): Int

    /**
     * Get Rust library version
     * @return Version string
     */
    @JvmStatic
    external fun rustGetVersion(): String

    /**
     * Test function to verify JNI connectivity
     * @param input Input value to double
     * @return Doubled value
     */
    @JvmStatic
    external fun rustTest(input: Int): Int

    /**
     * Set CPU affinity to big cores
     * @return 0 on success, -1 on failure
     */
    @JvmStatic
    external fun rustSetBigCoreAffinity(): Int

    /**
     * Copy frame buffer (simple version)
     * @param src Source address
     * @param dst Destination address
     * @param len Number of bytes to copy
     * @return 0 on success, -1 on failure
     */
    @JvmStatic
    external fun rustCopyFrameBuffer(src: Long, dst: Long, len: Int): Int

    /**
     * Copy frame buffer (SIMD accelerated)
     * @param src Source address
     * @param dst Destination address
     * @param len Number of bytes to copy
     * @return 0 on success, -1 on failure
     */
    @JvmStatic
    external fun rustCopyFrameBufferSimd(src: Long, dst: Long, len: Int): Int

    /**
     * Get optimal SIMD buffer size
     * @return Optimal buffer size in bytes
     */
    @JvmStatic
    external fun rustGetSimdOptimalSize(): Int

    /**
     * Copy file directly
     * @param srcPath Source file path
     * @param dstPath Destination file path
     * @return Number of bytes copied, or -1 on error
     */
    @JvmStatic
    external fun rustFileCopyDirect(srcPath: String, dstPath: String): Long

    /**
     * Copy file with progress callback
     * @param srcPath Source file path
     * @param dstPath Destination file path
     * @param bufferSize Buffer size to use
     * @param progressCallback Progress callback (not yet implemented)
     * @return Number of bytes copied, or -1 on error
     */
    @JvmStatic
    external fun rustFileCopyWithProgress(
        srcPath: String,
        dstPath: String,
        bufferSize: Int,
        progressCallback: Long
    ): Long

    /**
     * Get file size
     * @param path File path
     * @return File size in bytes, or -1 on error
     */
    @JvmStatic
    external fun rustFileSize(path: String): Long

    /**
     * Check if file exists
     * @param path File path
     * @return 1 if exists, 0 if not
     */
    @JvmStatic
    external fun rustFileExists(path: String): Int

    /**
     * Compute SHA1 hash of file
     * @param path File path
     * @return SHA1 hex string, or null on error
     */
    @JvmStatic
    external fun rustSha1File(path: String): String?

    /**
     * Verify file SHA1 against expected hash
     * @param path File path
     * @param expectedHex Expected SHA1 hex string
     * @return 1 if match, 0 if not, -1 on error
     */
    @JvmStatic
    external fun rustSha1Verify(path: String, expectedHex: String): Int

    /**
     * Compute SHA1 hash of bytes
     * @param data Input bytes
     * @return SHA1 hex string, or null on error
     */
    @JvmStatic
    external fun rustSha1Bytes(data: ByteArray): String?

    /**
     * Create directory recursively
     * @param path Directory path
     * @return 0 on success, -1 on error
     */
    @JvmStatic
    external fun rustCreateDirs(path: String): Int

    /**
     * Delete file
     * @param path File path
     * @return 0 on success, -1 on error
     */
    @JvmStatic
    external fun rustDeleteFile(path: String): Int

    /**
     * Load dynamic library using dlopen
     * @param name Library name or path
     * @return 1 on success, 0 on failure
     */
    @JvmStatic
    external fun rustDlopen(name: String): Boolean

    /**
     * Change current working directory
     * @param path New directory path
     * @return 0 on success, -1 on error
     */
    @JvmStatic
    external fun rustChdir(path: String): Int

    /**
     * Get current working directory
     * @return Current directory path, or null on error
     */
    @JvmStatic
    external fun rustGetcwd(): String?

    /**
     * Set LD_LIBRARY_PATH environment variable
     * @param ldLibraryPath New LD_LIBRARY_PATH value
     * @return 0 on success, -1 on error
     */
    @JvmStatic
    external fun rustSetLdLibraryPath(ldLibraryPath: String): Int

    /**
     * Initialize environment variables (hook for Java_exec_hooks)
     * @return 0 on success
     */
    @JvmStatic
    external fun initEnviron(): Int

    /**
     * Initialize game exit hook
     * @return 0 on success, -1 on error
     */
    @JvmStatic
    external fun initializeGameExitHook(): Int

    /**
     * Hook exec (stub for Java_exec_hooks compatibility)
     * @param cmd Command string
     * @return 0
     */
    @JvmStatic
    external fun hookExec(cmd: String): Int

    /**
     * Install LWJGL dlopen hook
     * @return 0 on success
     */
    @JvmStatic
    external fun installLwjglDlopenHook(): Int

    /**
     * Launch JVM using JLI_Launch
     * @param argsArray Arguments as byte array (null-separated strings)
     * @return Exit code
     */
    @JvmStatic
    external fun rustLaunchJVM(argsArray: ByteArray): Int
}