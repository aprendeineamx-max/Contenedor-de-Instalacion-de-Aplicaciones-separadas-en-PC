# Guía de Hooks (Detours + WinFSP)

Esta capa garantiza que los procesos dentro del contenedor lean y escriban únicamente en su carpeta aislada.

## Componentes
1. **HookEngine (`agent/src/runtime.rs`)**  
   - Calcula `HookPlan`: variables de entorno, montajes (`MountPlan`) y redirecciones (`PathRedirect`).
   - Crea directorios necesarios (`ProgramFiles`, `AppData`, `Temp`) antes de lanzar el proceso.

2. **NativeHookPipeline (`agent/src/hooks`)**  
   - Envuelve implementaciones específicas por plataforma.
   - En Windows con `--features native-hooks`, activa `DetoursHookManager` y hookea `CreateFileW`.
   - Redirige rutas a partir de `PathRedirect` (prefijos de `%APPDATA%`, `%LOCALAPPDATA%`, `%TEMP%`, etc.).

3. **WinFSP/Dokany**  
   - Usa los `MountPlan` generados para montar el árbol del contenedor como volumen virtual.
   - Permite exponer el contenedor como unidad (`X:`) o carpeta virtual para pruebas manuales.

## Requisitos para hooks nativos
1. Instalar **Detours** (Microsoft Research) y asegurarse de que las DLLs estén en el `PATH`.
2. Instalar **WinFSP** 2.0+ (o Dokany) y otorgar permisos para montar volúmenes por usuario.
3. Compilar el agent con `cargo run -p agent --features native-hooks`.
4. (Opcional) Ajustar variables:
   - `HOOKS_LOG=debug` para ver redirecciones en `tracing`.
   - `HOOKS_ALLOW_LIST=C:\CustomPath` para rutas extra (próximo soporte).

## Flujo resumido
1. `HookEngine::prepare` calcula `HookPlan`.
2. `HookEngine::activate` llama a `NativeHookPipeline::apply`.
3. `DetoursHookManager` reemplaza `CreateFileW` y reescribe rutas usando los prefijos capturados antes de cambiar el entorno.
4. WinFSP monta los directorios (`MountPlan`) para exponerlos como volumen o junction, útil para depuración.

## Próximos pasos
- Añadir hooks para `RegOpenKeyW` y APIs de servicios.
- Integrar `WinFsp.Launcher` para montar automáticamente `rootfs/` como `\\WinFSP\Containers\<id>`.
- Permitir listas blancas/negra configurables por contenedor (`config.yml`).

