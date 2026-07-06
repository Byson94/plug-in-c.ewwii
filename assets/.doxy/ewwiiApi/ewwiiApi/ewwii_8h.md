

# File ewwii.h



[**FileList**](files.md) **>** [**ewwii.h**](ewwii_8h.md)

[Go to the source code of this file](ewwii_8h_source.md)



* `#include <stdarg.h>`
* `#include <stdbool.h>`
* `#include <stdint.h>`
* `#include <stdlib.h>`















## Classes

| Type | Name |
| ---: | :--- |
| struct | [**CRuntimePaths**](structCRuntimePaths.md) <br> |
| struct | [**HostHandle**](structHostHandle.md) <br> |
| struct | [**RawMetadata**](structRawMetadata.md) <br> |


## Public Types

| Type | Name |
| ---: | :--- |
| typedef void(\* | [**CListenCallback**](#typedef-clistencallback)  <br> |
| typedef struct [**CRuntimePaths**](structCRuntimePaths.md) | [**CRuntimePaths**](#typedef-cruntimepaths)  <br> |
| typedef struct [**HostHandle**](structHostHandle.md) | [**HostHandle**](#typedef-hosthandle)  <br> |
| typedef struct [**RawMetadata**](structRawMetadata.md) | [**RawMetadata**](#typedef-rawmetadata)  <br> |




















## Public Functions

| Type | Name |
| ---: | :--- |
|  const char \* | [**ewwii\_api\_version**](#function-ewwii_api_version) (void) <br> |
|  void | [**ewwii\_emit**](#function-ewwii_emit) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* signal, const char \* data) <br>_Emit a message._  |
|  void | [**ewwii\_error**](#function-ewwii_error) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* msg) <br>_Log an error._  |
|  void | [**ewwii\_get\_runtime\_paths**](#function-ewwii_get_runtime_paths) (const struct [**HostHandle**](structHostHandle.md) \* handle, void(\*)(const struct [**HostHandle**](structHostHandle.md) \*, const struct [**CRuntimePaths**](structCRuntimePaths.md) \*) future\_handler) <br>_Get the runtime paths._  |
|  void | [**ewwii\_inject\_css**](#function-ewwii_inject_css) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* css, void(\*)(const struct [**HostHandle**](structHostHandle.md) \*, uint64\_t \*) future\_handler) <br>_Inject custom CSS._  |
|  void | [**ewwii\_inject\_nbcl**](#function-ewwii_inject_nbcl) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* nbcl) <br>_Inject nbcl._  |
|  void | [**ewwii\_listen**](#function-ewwii_listen) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* signal, CListenCallback callback) <br>_Listen to emissions._  |
|  void | [**ewwii\_log**](#function-ewwii_log) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* msg) <br>_Log a message._  |
|  void | [**ewwii\_register\_signal**](#function-ewwii_register_signal) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* name, const char \* initial) <br>_Register a signal (GlobalVar)_  |
|  void | [**ewwii\_remove\_css**](#function-ewwii_remove_css) (const struct [**HostHandle**](structHostHandle.md) \* handle, uint64\_t \* idx\_ptr) <br>_Remove an injected CSS._  |
|  void | [**ewwii\_signal\_value**](#function-ewwii_signal_value) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* name, void(\*)(const struct [**HostHandle**](structHostHandle.md) \*, const char \*) future\_handler) <br>_Get the value of a signal (GlobalVar)_  |
|  void | [**ewwii\_update\_signal**](#function-ewwii_update_signal) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* name, const char \* value) <br>_Update the value of a signal (GlobalVar)_  |
|  void | [**ewwii\_warn**](#function-ewwii_warn) (const struct [**HostHandle**](structHostHandle.md) \* handle, const char \* msg) <br>_Log a warning._  |
|  void | [**plugin\_init**](#function-plugin_init) (const struct [**HostHandle**](structHostHandle.md) \* host) <br> |
|  struct [**RawMetadata**](structRawMetadata.md) | [**plugin\_metadata**](#function-plugin_metadata) (void) <br> |




























## Public Types Documentation




### typedef CListenCallback 

```C++
typedef void(* CListenCallback) (const char *, const char *);
```




<hr>



### typedef CRuntimePaths 

```C++
typedef struct CRuntimePaths CRuntimePaths;
```



Paths to important files or directories like `config_dir`. 


        

<hr>



### typedef HostHandle 

```C++
typedef struct HostHandle HostHandle;
```



Host handle required for calling back to ewwii. 


        

<hr>



### typedef RawMetadata 

```C++
typedef struct RawMetadata RawMetadata;
```



Metadata of the plugin to register. 


        

<hr>
## Public Functions Documentation




### function ewwii\_api\_version 

```C++
const char * ewwii_api_version (
    void
) 
```




<hr>



### function ewwii\_emit 

_Emit a message._ 
```C++
void ewwii_emit (
    const struct HostHandle * handle,
    const char * signal,
    const char * data
) 
```



Emit a message which other plugins can see and work with the provided data.




**Parameters:**


* `handle` The host handle @signal The signal to emit @data The data to attach with the signal 




        

<hr>



### function ewwii\_error 

_Log an error._ 
```C++
void ewwii_error (
    const struct HostHandle * handle,
    const char * msg
) 
```



Log an error to ewwii with the appropriate plugin ID visible.




**Parameters:**


* `handle` The host handle 
* `msg` The message to log 




        

<hr>



### function ewwii\_get\_runtime\_paths 

_Get the runtime paths._ 
```C++
void ewwii_get_runtime_paths (
    const struct HostHandle * handle,
    void(*)(const struct HostHandle *, const struct CRuntimePaths *) future_handler
) 
```



Get the runtime paths like the configuration directory, socket file, etc.




**Parameters:**


* `handle` The host handle @future\_handler The function to call when the [**CRuntimePaths**](structCRuntimePaths.md) are resolved 




        

<hr>



### function ewwii\_inject\_css 

_Inject custom CSS._ 
```C++
void ewwii_inject_css (
    const struct HostHandle * handle,
    const char * css,
    void(*)(const struct HostHandle *, uint64_t *) future_handler
) 
```



Inject CSS into the core ewwii engine and handle the resulting CSS ID.




**Parameters:**


* `handle` The host handle 
* `css` The css string to inject @future\_handler A function to call when the CSS ID is resolved 




        

<hr>



### function ewwii\_inject\_nbcl 

_Inject nbcl._ 
```C++
void ewwii_inject_nbcl (
    const struct HostHandle * handle,
    const char * nbcl
) 
```



Inject nbcl into ewwii.




**Parameters:**


* `handle` The host handle 
* `nbcl` The nbcl code to inject 




        

<hr>



### function ewwii\_listen 

_Listen to emissions._ 
```C++
void ewwii_listen (
    const struct HostHandle * handle,
    const char * signal,
    CListenCallback callback
) 
```



Listen to emissions made by other plugins and ewwii itself.




**Parameters:**


* `handle` The host handle 
* `signal` The signal to listen to 
* `callback` The function to call when emission is found 




        

<hr>



### function ewwii\_log 

_Log a message._ 
```C++
void ewwii_log (
    const struct HostHandle * handle,
    const char * msg
) 
```



Log a message to ewwii with the appropriate plugin ID visible.




**Parameters:**


* `handle` The host handle 
* `msg` The message to log 




        

<hr>



### function ewwii\_register\_signal 

_Register a signal (GlobalVar)_ 
```C++
void ewwii_register_signal (
    const struct HostHandle * handle,
    const char * name,
    const char * initial
) 
```



Register a signal (GlobalVar) to ewwii which can be accessed from configuration.




**Parameters:**


* `handle` The host handle 
* `name` The name of the signal 
* `initial` The initial value of the signal 




        

<hr>



### function ewwii\_remove\_css 

_Remove an injected CSS._ 
```C++
void ewwii_remove_css (
    const struct HostHandle * handle,
    uint64_t * idx_ptr
) 
```



Remove an injected CSS from ewwii using the resolved CSS ID.




**Parameters:**


* `handle` The host handle 
* `idx_ptr` The pointer to the resolved CSS ID which is to be removed 




        

<hr>



### function ewwii\_signal\_value 

_Get the value of a signal (GlobalVar)_ 
```C++
void ewwii_signal_value (
    const struct HostHandle * handle,
    const char * name,
    void(*)(const struct HostHandle *, const char *) future_handler
) 
```



Get the value of a signal (GlobalVar) and do callback.




**Parameters:**


* `handle` The host handle 
* `name` The name of the signal to get value of 
* `future_handler` The function to call back to after resolving value 




        

<hr>



### function ewwii\_update\_signal 

_Update the value of a signal (GlobalVar)_ 
```C++
void ewwii_update_signal (
    const struct HostHandle * handle,
    const char * name,
    const char * value
) 
```



Update the value of a signal (Global).




**Parameters:**


* `handle` The host handle 
* `name` The name of the signal to update 
* `value` The value to set 




        

<hr>



### function ewwii\_warn 

_Log a warning._ 
```C++
void ewwii_warn (
    const struct HostHandle * handle,
    const char * msg
) 
```



Log a warning to ewwii with the appropriate plugin ID visible.




**Parameters:**


* `handle` The host handle 
* `msg` The message to log 




        

<hr>



### function plugin\_init 

```C++
void plugin_init (
    const struct HostHandle * host
) 
```




<hr>



### function plugin\_metadata 

```C++
struct RawMetadata plugin_metadata (
    void
) 
```




<hr>

------------------------------
The documentation for this class was generated from the following file `ewwii.h`

