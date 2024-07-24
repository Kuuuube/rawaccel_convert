# Rawaccel Convert

Tool for converting rawaccel curve settings to points. GUI version: [Rawaccel Convert GUI](https://github.com/Kuuuube/rawaccel-convert-gui).

# CLI

```
rawaccel_convert CURVE_TYPE
```

Filling all arguments is not required. Some curves conditionally change what they use based on other variables (ex: the `Cap Type`).

## Global Args

- `--pointscaling` (Default: `sens`): `sens`, `velocity`, `libinput`, or `libinputdebug`.

    Rawaccel's first graph uses `sens`, the second graph uses `velocity`.

    Libinput uses `libinput`. [Libinput custom accel curve setup](./docs/libinput.md).

    To output libinput values matching the format of `velocity` use `libinputdebug`. The output of `libinputdebug` cannot be used in libinput directly.

- `--pointcount` (Default: `64`): The number of points to include in the output.

    The maximum points libinput allows is `64`.

- `--dpi` (Default: `1200`): Your mouse DPI. Controls the max input speed to calculate for.

- `--sens` (Default: `1`): The `Sens Multiplier` field in rawaccel.

- `--gain` (Default: `true`): `true` or `false`. The `Gain` checkbox in rawaccel.

## Curve types

### Linear

- `--accel` (Default: `0.005`) The `Acceleration` field in rawaccel.

- `--captype` (Default: `output`) `output`, `input`, or `both`. The `Cap Type` field in rawaccel.

- `--capin` (Default: `15`) The `Cap: Input` field in rawaccel.

- `--capout` (Default: `1.5`) The `Cap: Output` field in rawaccel.

- `--offsetin` (Default: `0`) The `Input Offset` field in rawaccel.

### Classic

- `--accel` (Default: `0.005`) The `Acceleration` field in rawaccel.

- `--captype` (Default: `output`) `output`, `input`, or `both`. The `Cap Type` field in rawaccel.

- `--capin` (Default: `15`) The `Cap: Input` field in rawaccel.

- `--capout` (Default: `1.5`) The `Cap: Output` field in rawaccel.

- `--offsetin` (Default: `0`) The `Input Offset` field in rawaccel.

- `--power` (Default: `2`) The `Power` field in rawaccel.

### Jump

- `--smooth` (Default: `0.5`) The `Smooth` field in rawaccel.

- `--input` (Default: `15`) The `Input` field in rawaccel.

- `--output` (Default: `1.5`) The `Output` field in rawaccel.

### Natural

- `--decay` (Default: `0.1`) The `Decay Rate` field in rawaccel.

- `--offsetin` (Default: `0`) The `Input Offset` field in rawaccel.

- `--limit` (Default: `1.5`) The `Limit` field in rawaccel.

### Synchronous

- `--gamma` (Default: `1`) The `Gamma` field in rawaccel.

- `--smooth` (Default: `0.5`) The `Smooth` field in rawaccel.

- `--motivity` (Default: `1.5`) The `Motivity` field in rawaccel.

- `--syncspeed` (Default: `5`) The `SyncSpeed` field in rawaccel.

### Power

- `--scale` (Default: `1`) The `Scale` field in rawaccel.

- `--captype` (Default: `output`) `output`, `input`, or `both`. The `Cap Type` field in rawaccel.

- `--capin` (Default: `15`) The `Cap: Input` field in rawaccel.

- `--capout` (Default: `1.5`) The `Cap: Output` field in rawaccel.

- `--exponent` (Default: `0.05`) The `Exponent` field in rawaccel.

- `--offsetout` (Default: `0`) The `Output Offset` field in rawaccel.

## Examples

```
rawaccel_convert classic --sens=0.25 --accel=0.0315 --captype=input --capin=34 --power=2.5 --dpi=1600
```
