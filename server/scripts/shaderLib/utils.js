'use strict';


function blur(rwfbo, n = 1) {
    for (let i = 0; i < n; i++) {
        SHADERS.blurV.bind();
        gl.uniform1i(SHADERS.blurV.uniforms.u_tex, rwfbo.read.texture.attach(0));
        blit(rwfbo.write);
        rwfbo.swap();
        SHADERS.blurH.bind();
        gl.uniform1i(SHADERS.blurH.uniforms.u_tex, rwfbo.read.texture.attach(0));
        blit(rwfbo.write);
        rwfbo.swap();
    }
}
