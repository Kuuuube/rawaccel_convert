# Rawaccel Convert

Tool for converting rawaccel curve settings to points. GUI version: [Rawaccel Convert GUI](https://github.com/Kuuuube/rawaccel-convert-gui).

# CLI

```
rawaccel_convert CURVE_TYPE [OPTIONS]...
```

Filling all arguments is not required. Some curves conditionally change what they use based on other variables (ex: the `Cap Type`).

> [!WARNING]
> There is nearly zero input validation for both args and values. If you get a curve that doesn't make sense, check that you haven't made a typo. Visualizing your inputs with the [Rawaccel Convert GUI](https://github.com/Kuuuube/rawaccel-convert-gui) may also help.

## Global Args

These args apply regardless of the curve type you select.

- `--pointscaling` (Default: `sens`): `sens`, `velocity`, `libinput`, `libinputdebug`, `lookupvelocity`, or `lookupsens`.

    Rawaccel's first graph uses `sens`, the second graph uses `velocity`.

    Libinput uses `libinput`. [Libinput custom accel curve setup](./docs/libinput.md).

    To output libinput values matching the format of `velocity` use `libinputdebug`. The output of `libinputdebug` cannot be used in libinput directly.

    The `lookupvelocity` option outputs in the format of rawaccel's Look Up Table curve with the velocity setting.

    The `lookupsens` option outputs in the format of rawaccel's Look Up Table curve with the velocity setting.

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

### Motivity

Note: Motivity has been superceded by `Synchronous`

- `--growthrate` (Default: `1`) The `Growth Rate` field in rawaccel.

- `--motivity` (Default: `1.5`) The `Motivity` field in rawaccel.

- `--midpoint` (Default: `5`) The `Midpoint` field in rawaccel.

### Power

- `--scale` (Default: `1`) The `Scale` field in rawaccel.

- `--captype` (Default: `output`) `output`, `input`, or `both`. The `Cap Type` field in rawaccel.

- `--capin` (Default: `15`) The `Cap: Input` field in rawaccel.

- `--capout` (Default: `1.5`) The `Cap: Output` field in rawaccel.

- `--exponent` (Default: `0.05`) The `Exponent` field in rawaccel.

- `--offsetout` (Default: `0`) The `Output Offset` field in rawaccel.

### Lookup

- `--points` (Default: ` `) The `Look Up Table` text box in rawaccel. Split X/Y axis accel is not supported.

    Do not include whitespace in your list of points.

    You may need to wrap the points in quotes to avoid some shells interpreting `;` as a command.

    Example:
    ```
    --points="1.505035,0.85549892;4.375,3.30972978;13.51,15.17478447;140,354.7026875;"
    ```

- `--applyas` (Default: `sens`) `sens` or `velocity`. The `Apply as` field in rawaccel.

## Examples

```
rawaccel_convert classic --sens=0.25 --accel=0.0315 --captype=input --capin=34 --power=2.5 --dpi=1600
```
