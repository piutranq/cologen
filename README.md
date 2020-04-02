# cologen

The simple color scheme configuration generator,
inspired by [base16-builder](https://github.com/base16-builder/base16-builder).

## Install

- Install from sources
```sh
git clone https://github.com/piutranq/cologen
cargo install --path ./cologen
```

- Install from crates.io
```sh
cargo install cologen
```

## Usage
```plain
Usage) 
    [TEMPLATE] | cologen [SCHEME] > [OUTPUT]

    [TEMPLATE]: Template configuration file for target.
                It must be input at stdin.
    [SCHEME]: Path of color scheme file. (YAML format)
    [OUTPUT]: The generated configuration is printed at stdout.

Example)
    cat $XDG_CONFIG_HOME/cologen/templates/rofi.template \
    | cologen $XDG_CONFIG_HOME/cologen/schemes/gruvbox-dark.yaml \
    > $XDG_CONFIG_HOME/rofi/color.rasi

```

## Color scheme
The color scheme file follows the YAML format.
see below example, or `example/scheme.yaml`.

The example is a color scheme from [gruvbox](https://github.com/morhetz/gruvbox)

```yaml
name: "gruvbox-dark"
color:
  # Monochromes
  grey0: [ 0x28, 0x28, 0x28 ] # bg0
  grey1: [ 0x3C, 0x38, 0x36 ] # bg1
  grey2: [ 0x66, 0x5C, 0x54 ] # bg3
  grey3: [ 0xBD, 0xAE, 0x93 ] # fg3
  grey4: [ 0xEB, 0xDB, 0xB2 ] # fg1
  grey5: [ 0xFB, 0xF1, 0xC7 ] # fg0

  # Chromatics
  red:      [ 0xFB, 0x49, 0x34 ] # red (bold)
  green:    [ 0xB8, 0xBB, 0x26 ] # green (bold)
  yellow:   [ 0xFA, 0xBD, 0x2F ] # yellow (bold)
  blue:     [ 0x83, 0xA5, 0x98 ] # blue (bold)
  magenta:  [ 0xD3, 0x86, 0x9B ] # purple (bold)
  cyan:     [ 0x8E, 0xC0, 0x7C ] # aqua (bold)

```

cologen has not the standard or guideline for color naming.
it just replaces the substitutes in the template to the actual color codes
based on the color name.


## Formats

### Template
- The template file has the same format as the target config file,
just color codes are only replaced with substitutes.

- Substitutes have the following format, `@[color_name:color_code_format]`

- The following text is the example of the template
for [rofi](https://github.com/davatorium/rofi). see `example/template`

```css
/* color config example for rofi */
*
{
    /* Monochrome */
    grey0: @[grey0:#%xR%xG%xB];
    grey1: @[grey1:#%xR%xG%xB];
    grey2: @[grey2:#%xR%xG%xB];
    grey3: @[grey3:#%xR%xG%xB];
    grey4: @[grey4:#%xR%xG%xB];
    grey5: @[grey5:#%xR%xG%xB];

    /* Chromatics */
    red: @[red:#%xR%xG%xB];
    green: @[green:#%xR%xG%xB];
    yellow: @[yellow:#%xR%xG%xB];
    blue: @[blue:#%xR%xG%xB];
    magenta: @[magenta:#%xR%xG%xB];
    cyan: @[cyan:#%xR%xG%xB];

    /* Special Purpose */
    empty: rgba(0, 0, 0, 0);
    bg: @[grey0:rgba(%dR, %dG, %dB, %.A)];
    fg: @[grey4:#%xR%xG%xB];
    fgbold: bold underline @[grey5:#%xR%xG%xB];
}
```

- The template example is converted to the following. see `example/output`
```css
/* color config example for rofi */
*
{
    /* Monochrome */
    grey0: #282828;
    grey1: #3c3836;
    grey2: #665c54;
    grey3: #bdae93;
    grey4: #ebdbb2;
    grey5: #fbf1c7;

    /* Chromatics */
    red: #fb4934;
    green: #b8bb26;
    yellow: #fabd2f;
    blue: #83a598;
    magenta: #d3869b;
    cyan: #8ec07c;

    /* Special Purpose */
    empty: rgba(0, 0, 0, 0);
    bg: rgba(40, 40, 40, 1.00);
    fg: #ebdbb2;
    fgbold: bold underline #fbf1c7;
}
```

### Color name
- Color name can use character `A` to `Z`, `a` to `z`, `0` to `9`, and `_`.
- The followings are the example:
    - `Grey2` is valid (Upper case is allowed)
    - `foreground_highlighted` is valid (`_` is only allowed special character)
    - `회색` is invalid (non-roman character used)
    - `red-dimmed` is invalid (invalid special character used)

### Color code format
-  Color code format uses `%` for escape character.
non-escaped expressions are not replaced.
- `%%` will be replaced to `%`
- `%dR`, `%dG`, `%dB`, `%dA` will be replaced to 0 to 255
- `%pR`, `%pG`, `%pB`, `%pA` will be replaced to 0 to 100
- `%.R`, `%.G`, `%.B`, `%.A` will be replaced to 0.00 to 1.00
- `%xR`, `%xG`, `%xB`, `%xA` will be replaced to 00 to FF

