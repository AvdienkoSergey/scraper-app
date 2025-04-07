#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
  <application>
    <!-- Windows 10 and Windows Server 2016 -->
    <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
    <!-- Windows 8.1 and Windows Server 2012 R2 -->
    <supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}"/>
    <!-- Windows 8 and Windows Server 2012 -->
    <supportedOS Id="{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}"/>
    <!-- Windows 7 and Windows Server 2008 R2 -->
    <supportedOS Id="{35138b9a-5d96-4fbd-8e2d-a2440225f93a}"/>
    <!-- Windows Vista and Windows Server 2008 -->
    <supportedOS Id="{e2011457-1546-43c5-a5fe-008deee3d3f0}"/>
    <!-- Windows XP and Windows Server 2003 -->
    <supportedOS Id="{1e9b9121-e457-4123-ad60-ad1c8a501c14}"/>
  </application>
</compatibility>
</assembly>
"#);
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    // Nothing to do for non-Windows platforms
} 