include (${CMAKE_CURRENT_LIST_DIR}/common.cmake)
include (${CMAKE_CURRENT_LIST_DIR}/ios-helpers.cmake)


########
init_apple_environment()
set(CMAKE_SYSTEM_NAME iOS)
set(CMAKE_OSX_DEPLOYMENT_TARGET 13.00 CACHE INTERNAL "")
set(CMAKE_OSX_ARCHITECTURES "arm64" CACHE INTERNAL "")
set(CMAKE_IOS_INSTALL_COMBINED FALSE)
set(SDK_NAME iphoneos)
set(CMAKE_C_COMPILER_TARGET arm64-apple-ios${CMAKE_OSX_DEPLOYMENT_TARGET})
set(CMAKE_CXX_COMPILER_TARGET arm64-apple-ios${CMAKE_OSX_DEPLOYMENT_TARGET})
set(CMAKE_LIBRARY_ARCHITECTURE arm64-apple-ios${CMAKE_OSX_DEPLOYMENT_TARGET})
execute_process(COMMAND xcodebuild -version -sdk iphone Path
    OUTPUT_VARIABLE CMAKE_OSX_SYSROOT
    OUTPUT_STRIP_TRAILING_WHITESPACE
    ERROR_VARIABLE _execError
    ERROR_QUIET
)
if(_execError)
    message(FATAL_ERROR "seeking CMAKE_OSX_SYSROOT for iphone failed with error ${_execError}")
endif()


# Fix for PThread library not in path
set(CMAKE_THREAD_LIBS_INIT "-lpthread")
set(CMAKE_HAVE_THREADS_LIBRARY YES)
set(CMAKE_USE_WIN32_THREADS_INIT NO)
set(CMAKE_USE_PTHREADS_INIT YES)

set(_rustTriple "aarch64-apple-ios")
