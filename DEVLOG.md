- Learned that cargo does not 'pin' package versions unless you explicitly ask ("=-0.0")
- Cargo will automatically find a dependency by package name when provided a git -po
- Fish command to add rustc to path:
    ```
    set -U fish_user_paths $HOME/.cargo/bin $fish_user_paths
    ```
- I have been referencing the [N-API implementation](https://www.prisma.io/docs/concepts/components/prisma-engines/query-engine#enable-the-node-api-n-api-preview)
- [N-API TS Engine reference](https://github.com/prisma/prisma/blob/fc7b272bfa7ee030c2c8dbede73e0da4963f4c67/packages/engine-core/src/library/LibraryEngine.ts#L81)