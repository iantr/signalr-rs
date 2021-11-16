OpenSSL needs to be installed for this project to build.

I was successful following these steps:

	1. clone vcpkg
	2. open directory where you've cloned vcpkg
	3. run ./bootstrap-vcpkg.bat
	4. run ./vcpkg.exe install openssl-windows:x64-windows
	5. run ./vcpkg.exe install openssl:x64-windows-static
	6. run ./vcpkg.exe integrate install
	7. run set VCPKGRS_DYNAMIC=1 (or simply set it as your environment variable)
    8. set user environment variable OPENSSL_DIR="<vcpkg>\installed\x64-windows-static"
