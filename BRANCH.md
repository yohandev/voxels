```rust
struct MenuScene;
struct GameScene;
struct PauseScene;

struct SFoo;

impl System for SFoo
{
    // default
    const SCENES: SysMask = SysMask::all();
    // or
    const SCENES: SysMask = <(PauseScene, MenuScene)>::blacklist();
    // or
    const SCENES: SysMask = <(GameScene,)>::whitelist();

    /// register resources and event handlers
    fn register(cmd: Cmd, reg: SubRegistry, )
    {
        
    }
}
```