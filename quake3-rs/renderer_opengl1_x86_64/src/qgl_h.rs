pub type BindTextureproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: crate::stdlib::GLuint) -> ();
pub type BlendFuncproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: crate::stdlib::GLenum) -> ();
pub type ClearColorproc = unsafe extern "C" fn(
    _: crate::stdlib::GLclampf,
    _: crate::stdlib::GLclampf,
    _: crate::stdlib::GLclampf,
    _: crate::stdlib::GLclampf,
) -> ();
pub type Clearproc = unsafe extern "C" fn(_: crate::stdlib::GLbitfield) -> ();
pub type ColorMaskproc = unsafe extern "C" fn(
    _: crate::stdlib::GLboolean,
    _: crate::stdlib::GLboolean,
    _: crate::stdlib::GLboolean,
    _: crate::stdlib::GLboolean,
) -> ();
pub type CullFaceproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type DepthFuncproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type DepthMaskproc = unsafe extern "C" fn(_: crate::stdlib::GLboolean) -> ();
pub type Disableproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type Enableproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type Finishproc = unsafe extern "C" fn() -> ();
pub type ReadPixelsproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: *mut libc::c_void,
) -> ();
pub type Scissorproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
) -> ();
pub type TexImage2Dproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: *const libc::c_void,
) -> ();
pub type TexParameterfproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLfloat,
) -> ();
pub type TexSubImage2Dproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: *const libc::c_void,
) -> ();
pub type Viewportproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
) -> ();
pub type ClearStencilproc = unsafe extern "C" fn(_: crate::stdlib::GLint) -> ();
pub type GetErrorproc = unsafe extern "C" fn() -> crate::stdlib::GLenum;
pub type StencilFuncproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLuint,
) -> ();
pub type StencilMaskproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type StencilOpproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
) -> ();
pub type DeleteTexturesproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *const crate::stdlib::GLuint) -> ();
pub type GenTexturesproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *mut crate::stdlib::GLuint) -> ();
pub type TexParameteriproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
) -> ();
pub type GetIntegervproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: *mut crate::stdlib::GLint) -> ();
pub type DrawElementsproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLenum,
    _: *const libc::c_void,
) -> ();
pub type PolygonOffsetproc =
    unsafe extern "C" fn(_: crate::stdlib::GLfloat, _: crate::stdlib::GLfloat) -> ();
pub type GetBooleanvproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: *mut crate::stdlib::GLboolean) -> ();
pub type Translatefproc = unsafe extern "C" fn(
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type LineWidthproc = unsafe extern "C" fn(_: crate::stdlib::GLfloat) -> ();
pub type CopyTexSubImage2Dproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
) -> ();
pub type DrawArraysproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
) -> ();
pub type Flushproc = unsafe extern "C" fn() -> ();
pub type GetStringproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum) -> *const crate::stdlib::GLubyte;
pub type AlphaFuncproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: crate::stdlib::GLclampf) -> ();
pub type LoadIdentityproc = unsafe extern "C" fn() -> ();
pub type LoadMatrixfproc = unsafe extern "C" fn(_: *const crate::stdlib::GLfloat) -> ();
pub type MatrixModeproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type TexEnvfproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLfloat,
) -> ();
pub type PopMatrixproc = unsafe extern "C" fn() -> ();
pub type PushMatrixproc = unsafe extern "C" fn() -> ();
pub type Color4fproc = unsafe extern "C" fn(
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type EnableClientStateproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type ShadeModelproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type ColorPointerproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: *const libc::c_void,
) -> ();
pub type DisableClientStateproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type TexCoordPointerproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: *const libc::c_void,
) -> ();
pub type VertexPointerproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: *const libc::c_void,
) -> ();
pub type DepthRangeproc =
    unsafe extern "C" fn(_: crate::stdlib::GLclampd, _: crate::stdlib::GLclampd) -> ();
pub type DrawBufferproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type PolygonModeproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: crate::stdlib::GLenum) -> ();
pub type ClearDepthproc = unsafe extern "C" fn(_: crate::stdlib::GLclampd) -> ();
pub type Beginproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type ClipPlaneproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: *const crate::stdlib::GLdouble) -> ();
pub type Color3fproc = unsafe extern "C" fn(
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type Endproc = unsafe extern "C" fn() -> ();
pub type Orthoproc = unsafe extern "C" fn(
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
) -> ();
pub type TexCoord2fproc =
    unsafe extern "C" fn(_: crate::stdlib::GLfloat, _: crate::stdlib::GLfloat) -> ();
pub type Vertex2fproc =
    unsafe extern "C" fn(_: crate::stdlib::GLfloat, _: crate::stdlib::GLfloat) -> ();
pub type Vertex3fvproc = unsafe extern "C" fn(_: *const crate::stdlib::GLfloat) -> ();
pub type ArrayElementproc = unsafe extern "C" fn(_: crate::stdlib::GLint) -> ();
pub type Color4ubvproc = unsafe extern "C" fn(_: *const crate::stdlib::GLubyte) -> ();
pub type TexCoord2fvproc = unsafe extern "C" fn(_: *const crate::stdlib::GLfloat) -> ();
pub type Vertex3fproc = unsafe extern "C" fn(
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type Frustumproc = unsafe extern "C" fn(
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
    _: crate::stdlib::GLdouble,
) -> ();
pub type ClearDepthfproc = unsafe extern "C" fn(_: crate::stdlib::GLclampf) -> ();
pub type DepthRangefproc =
    unsafe extern "C" fn(_: crate::stdlib::GLclampf, _: crate::stdlib::GLclampf) -> ();
pub type ClipPlanefproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: *const crate::stdlib::GLfloat) -> ();
pub type Frustumfproc = unsafe extern "C" fn(
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type Orthofproc = unsafe extern "C" fn(
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type ActiveTextureproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type CompressedTexImage2Dproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: *const libc::c_void,
) -> ();
pub type CompressedTexSubImage2Dproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: *const libc::c_void,
) -> ();
pub type BindBufferproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: crate::stdlib::GLuint) -> ();
pub type DeleteBuffersproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *const crate::stdlib::GLuint) -> ();
pub type GenBuffersproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *mut crate::stdlib::GLuint) -> ();
pub type BufferDataproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizeiptr,
    _: *const libc::c_void,
    _: crate::stdlib::GLenum,
) -> ();
pub type BufferSubDataproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLintptr,
    _: crate::stdlib::GLsizeiptr,
    _: *const libc::c_void,
) -> ();
pub type AttachShaderproc =
    unsafe extern "C" fn(_: crate::stdlib::GLuint, _: crate::stdlib::GLuint) -> ();
pub type BindAttribLocationproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLuint,
    _: *const crate::stdlib::GLchar,
) -> ();
pub type CompileShaderproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type CreateProgramproc = unsafe extern "C" fn() -> crate::stdlib::GLuint;
pub type CreateShaderproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> crate::stdlib::GLuint;
pub type DeleteProgramproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type DeleteShaderproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type DetachShaderproc =
    unsafe extern "C" fn(_: crate::stdlib::GLuint, _: crate::stdlib::GLuint) -> ();
pub type DisableVertexAttribArrayproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type EnableVertexAttribArrayproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type GetActiveUniformproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLsizei,
    _: *mut crate::stdlib::GLsizei,
    _: *mut crate::stdlib::GLint,
    _: *mut crate::stdlib::GLenum,
    _: *mut crate::stdlib::GLchar,
) -> ();
pub type GetProgramivproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: *mut crate::stdlib::GLint,
) -> ();
pub type GetProgramInfoLogproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLsizei,
    _: *mut crate::stdlib::GLsizei,
    _: *mut crate::stdlib::GLchar,
) -> ();
pub type GetShaderivproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: *mut crate::stdlib::GLint,
) -> ();
pub type GetShaderInfoLogproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLsizei,
    _: *mut crate::stdlib::GLsizei,
    _: *mut crate::stdlib::GLchar,
) -> ();
pub type GetShaderSourceproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLsizei,
    _: *mut crate::stdlib::GLsizei,
    _: *mut crate::stdlib::GLchar,
) -> ();
pub type GetUniformLocationproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: *const crate::stdlib::GLchar,
) -> crate::stdlib::GLint;
pub type LinkProgramproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type ShaderSourceproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLsizei,
    _: *mut *const crate::stdlib::GLchar,
    _: *const crate::stdlib::GLint,
) -> ();
pub type UseProgramproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type Uniform1fproc =
    unsafe extern "C" fn(_: crate::stdlib::GLint, _: crate::stdlib::GLfloat) -> ();
pub type Uniform2fproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type Uniform3fproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type Uniform4fproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> ();
pub type Uniform1iproc =
    unsafe extern "C" fn(_: crate::stdlib::GLint, _: crate::stdlib::GLint) -> ();
pub type Uniform1fvproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: *const crate::stdlib::GLfloat,
) -> ();
pub type UniformMatrix4fvproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLboolean,
    _: *const crate::stdlib::GLfloat,
) -> ();
pub type ValidateProgramproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type VertexAttribPointerproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLboolean,
    _: crate::stdlib::GLsizei,
    _: *const libc::c_void,
) -> ();
pub type GetStringiproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLuint,
) -> *const crate::stdlib::GLubyte;
pub type GenQueriesproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *mut crate::stdlib::GLuint) -> ();
pub type DeleteQueriesproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *const crate::stdlib::GLuint) -> ();
pub type BeginQueryproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: crate::stdlib::GLuint) -> ();
pub type EndQueryproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type GetQueryObjectivproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: *mut crate::stdlib::GLint,
) -> ();
pub type GetQueryObjectuivproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: *mut crate::stdlib::GLuint,
) -> ();
pub type BindRenderbufferproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: crate::stdlib::GLuint) -> ();
pub type DeleteRenderbuffersproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *const crate::stdlib::GLuint) -> ();
pub type GenRenderbuffersproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *mut crate::stdlib::GLuint) -> ();
pub type RenderbufferStorageproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
) -> ();
pub type BindFramebufferproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum, _: crate::stdlib::GLuint) -> ();
pub type DeleteFramebuffersproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *const crate::stdlib::GLuint) -> ();
pub type GenFramebuffersproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *mut crate::stdlib::GLuint) -> ();
pub type CheckFramebufferStatusproc =
    unsafe extern "C" fn(_: crate::stdlib::GLenum) -> crate::stdlib::GLenum;
pub type FramebufferTexture2Dproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
) -> ();
pub type FramebufferRenderbufferproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLuint,
) -> ();
pub type GenerateMipmapproc = unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ();
pub type BlitFramebufferproc = unsafe extern "C" fn(
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLbitfield,
    _: crate::stdlib::GLenum,
) -> ();
pub type RenderbufferStorageMultisampleproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
) -> ();
pub type BindVertexArrayproc = unsafe extern "C" fn(_: crate::stdlib::GLuint) -> ();
pub type DeleteVertexArraysproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *const crate::stdlib::GLuint) -> ();
pub type GenVertexArraysproc =
    unsafe extern "C" fn(_: crate::stdlib::GLsizei, _: *mut crate::stdlib::GLuint) -> ();
pub type BindMultiTextureEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLuint,
) -> crate::stdlib::GLvoid;
pub type TextureParameterfEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLfloat,
) -> crate::stdlib::GLvoid;
pub type TextureParameteriEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
) -> crate::stdlib::GLvoid;
pub type TextureImage2DEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: *const libc::c_void,
) -> crate::stdlib::GLvoid;
pub type TextureSubImage2DEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: *const libc::c_void,
) -> crate::stdlib::GLvoid;
pub type CopyTextureSubImage2DEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
) -> crate::stdlib::GLvoid;
pub type CompressedTextureImage2DEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: *const libc::c_void,
) -> crate::stdlib::GLvoid;
pub type CompressedTextureSubImage2DEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: *const libc::c_void,
) -> crate::stdlib::GLvoid;
pub type GenerateTextureMipmapEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
) -> crate::stdlib::GLvoid;
pub type ProgramUniform1iEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLint,
) -> crate::stdlib::GLvoid;
pub type ProgramUniform1fEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLfloat,
) -> crate::stdlib::GLvoid;
pub type ProgramUniform2fEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> crate::stdlib::GLvoid;
pub type ProgramUniform3fEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> crate::stdlib::GLvoid;
pub type ProgramUniform4fEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
    _: crate::stdlib::GLfloat,
) -> crate::stdlib::GLvoid;
pub type ProgramUniform1fvEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: *const crate::stdlib::GLfloat,
) -> crate::stdlib::GLvoid;
pub type ProgramUniformMatrix4fvEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLboolean,
    _: *const crate::stdlib::GLfloat,
) -> crate::stdlib::GLvoid;
pub type NamedRenderbufferStorageEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
) -> crate::stdlib::GLvoid;
pub type NamedRenderbufferStorageMultisampleEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLsizei,
    _: crate::stdlib::GLsizei,
)
    -> crate::stdlib::GLvoid;
pub type CheckNamedFramebufferStatusEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
) -> crate::stdlib::GLenum;
pub type NamedFramebufferTexture2DEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLint,
) -> crate::stdlib::GLvoid;
pub type NamedFramebufferRenderbufferEXTproc = unsafe extern "C" fn(
    _: crate::stdlib::GLuint,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLenum,
    _: crate::stdlib::GLuint,
) -> crate::stdlib::GLvoid;
