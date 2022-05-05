# VL
A CLI to generate [Vega-Lite](https://vega.github.io/vega-lite/) specifications.

This project is currently a major work in progress so many parts are unrefined or may not work.

The overall goal of this tool is to be useful in quick shell scripts for visualizing data.
For anything more involved or long term, consider any of the more mature Vega-Lite bindings.

## Usage
A spec can either be fed data directly with a file:
```shell
vl 'data/cars.json' bar --x='f:Origin' --y='f:Miles_per_Gallon,t:q'
```
Or it can be piped in:
```shell
cat data/cars.json | vl bar --x='f:Origin' --y='f:Miles_per_Gallon,t:q'
```
Alternatively one can choose to not feed in data at all and just generate the encoding and mark type:
```shell
vl bar --x='f:Origin' --y='f:Miles_per_Gallon,t:q'
```
There is a comma-separated short hand for encodings to allow for quick plotting (of the format `key:value`):

|key               |value     |
| ---------------- | -------- |
|`a` or `aggregate`|`string`  |
|`b` or `bin`      |`bool`    |
|`f` or `field`    |`string`  |
|`t` or `type`     |`string`\*|
|`u` or `timeUnit` |`string`  |

alternatively any `key:value` pair should just be passed through as well.

\*For type there are also the following shorthands:
| char              | type         |
| ----------------- | ------------ |
|`q`                |`quantitative`|
|`t`                |`temporal`    |
|`o`                |`ordinal`     |
|`n`                |`nominal`     |
|`g`                |`geojson`     |

## Config
`vl` can make use of a config file located at `~/.config/vl/config.json`. The config file takes any values that [Vega-Lite's `config` parameter](https://vega.github.io/vega-lite/docs/config.html) takes.

If you would like to ignore your config file for a specific plot, you can plot with:
```shell
vl --bare [...]

```

---

This tool is particularly useful with a terminal that allows inline graphics such as [Kitty](https://sw.kovidgoyal.net/kitty/) along with the [Vega-Lite CLI](https://vega.github.io/vega-lite/usage/compile.html). With the above, one can run:
```shell
cat data/cars.json | vl bar --x='f:Origin' --y='f:Miles_per_Gallon,t:q' | vl2png | icat
```
where icat is replaced by whatever your terminal's graphic representation command is.
