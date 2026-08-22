# Native distribution investigation

- native module source: relative game VFS path or `SPRING_NATIVE_MODULE`
- VFS module path: copied to a temporary filesystem path before loading
- loader symbol checked before callbacks: `NativeModuleApiVersion`
- compatibility check: major version equality; module minor not newer than host
- callback ABI: engine-owned `NativeInterface` tables passed to the event client
- engine binary link: no explicit engine-library link in the loader contract
- ABI dependency: required
- native guest shared-library dependency closure: not yet measured with `ldd`
- result: claim plausible from loader design; distribution claim remains unverified
- restructure: none
- ABI policy: out of scope
- native environment model: out of scope
