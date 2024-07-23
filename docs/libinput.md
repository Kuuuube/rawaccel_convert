# Libinput

Applying a custom accel curve with libinput.

## X11

1. List your devices and find your mouse's ID:

    ```
    xinput list
    ```

2. The output will look something like this:

    ```
    ⎡ Virtual core pointer                      id=2    [master pointer  (3)]
    ⎜   ↳ Gaming Mouse Keyboard                 id=14   [slave  pointer  (2)]
    ⎜   ↳ Gaming Mouse                          id=15   [slave  pointer  (2)]
    ⎣ Virtual core keyboard                     id=3    [master keyboard (2)]
    ...
    ```

    In this example, the ID is 15.

3. Enable the custom accel profile:

    ```
    xinput set-prop {mouse_id} "libinput Accel Profile Enabled" 0, 0, 1
    ```

    Sometimes this may not update properly right away. Try running it a second time if that happens.

    **Replace `{mouse_id}` with your mouse's ID.**

4. Set the correct step size:

    ```
    xinput set-prop {mouse_id} "libinput Accel Custom Motion Step" 1.0
    ```

5. Set the custom accel curve points:

    ```
    xinput set-prop {mouse_id} "libinput Accel Custom Motion Points" {accel_points}
    ```

    **Replace `{accel_points}` with the output of `rawaccel_convert` with `--pointscaling=libinput` flag.**

6. To check everything has been set as expected, run:

    ```
    xinput list-props {mouse_id}
    ```

### Reverting these changes


1. To disable the custom accel curve, run:

    ```
    xinput set-prop {mouse_id} "libinput Accel Profile Enabled" 1, 0, 0
    ```

    Sometimes this may not update properly right away. Try running it a second time if that happens.

2. To reset the custom accel curve step size, run:

    ```
    xinput set-prop {mouse_id} "libinput Accel Custom Motion Step" 1.0
    ```

3. To reset the custom accel curve points, run:

    ```
    xinput set-prop {mouse_id} "libinput Accel Custom Motion Points" 0.0, 0.5, 1.0
    ```

## Wayland

I'm currently unaware of any methods for applying a custom accel curve on wayland. If you know how, please contact me through github issues or make a PR on this repo adding a method for this.

Libinput pointer acceleration docs: https://wayland.freedesktop.org/libinput/doc/latest/pointer-acceleration.html
