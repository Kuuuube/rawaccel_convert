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
    xinput set-prop {mouse_id} "libinput Accel Custom Motion Step" {libinput_step}
    ```

    **Replace `{libinput_step}` with the output of `rawaccel_convert` with `--pointscaling=libinput` flag or the steps box in the GUI.**

    Example:

    ```
    xinput set-prop 15 "libinput Accel Custom Motion Step" 0.7936507936507936
    ```

5. Set the custom accel curve points:

    ```
    xinput set-prop {mouse_id} "libinput Accel Custom Motion Points" {accel_points}
    ```

    **Replace `{accel_points}` with the output of `rawaccel_convert` with `--pointscaling=libinput` flag or the points box in the GUI.**

    Example:

    ```
    xinput set-prop 15 "libinput Accel Custom Motion Points" 0 0.7938491877010421 1.5884238735278087 2.3840450381269274 3.1809517842111235 3.9793445328211408 4.779399472784817 5.581275720164609 6.385119508445442 7.191066897067502 7.99924564380902 8.809776563821584 9.622774553395008 10.438349384095387 11.256606333577478 12.077646696596608 12.901568205867056 13.728465383592614 14.558429838686147 15.39155052075281 16.227913939165415 17.067604353603905 17.910703941008556 18.75729294284408 19.607449795780834 20.461251248296083 21.318772465232396 22.180087121986197 23.045267489711932 23.9143845126981 24.787507878886554 25.664706084357086 26.546046492476368 27.431595388309876 28.321418028811536 29.21557868923598 30.11414070616011 31.0171665174508 31.924717699473998 32.83685500180465 33.753638379666405 34.675127024303656 35.6013793914659 36.53245322816466 37.46840559784628 38.40929290410878 39.355170913078155 40.306094774547574 41.262119041973584 42.223297691413705 43.18968413948235 44.1613312603949 45.138291402163524 46.12061640200252 47.108357600996406 48.10156585807902 49.10029156336821 50.10458465089703 51.11449461077875 52.13007050084062 53.151360957758115 54.17841420771926 55.21127807664629 56.25
    ```

6. To check everything has been set as expected, run:

    ```
    xinput list-props {mouse_id}
    ```

### Rotation

- Input rotation can be set in degrees (0-359) using this command:

    ```
    xinput set-prop {mouse_id} "libinput Rotation Angle" {rotation_angle}
    ```

    Example:

    ```
    xinput set-prop 15 "libinput Rotation Angle" 90
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
    xinput set-prop {mouse_id} "libinput Accel Custom Motion Points" 0.0, 1.0
    ```

## Hyprland

### Applying to all devices

1. Set the following setting in the input section of your config file:

    ```
    input {
        accel_profile = custom {libinput_step} {accel_points}
        ...
    }
    ```

    **Replace `{libinput_step}` and `{accel_points}` with the output of `rawaccel_convert` with `--pointscaling=libinput` flag or the steps box in the GUI.**

2. It should automatically reload and apply your settings. If it does not, run:

    ```
    hyprctl reload
    ```

### Applying to a single device

1. List your device names:

    ```
    hyprctl devices
    ```

2. Set the following setting in the input section of your config file:

    ```
    device:{device_name} {
        accel_profile = custom {libinput_step} {accel_points}
        ...
    }
    ```

    **Replace `{device_name}` with your device's name.**

3. It should automatically reload and apply your settings. If it does not, run:

    ```
    hyprctl reload
    ```

### Reverting these changes

1. Remove the code added to the config file in the above steps.

2. It should automatically reload and apply your settings. If it does not, run:

    ```
    hyprctl reload
    ```

## Other Wayland

Libinput pointer acceleration docs: https://wayland.freedesktop.org/libinput/doc/latest/pointer-acceleration.html
