# Note

## Merge two pdfs

```shell
gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -sOutputFile=merge.pdf a.pdf b.pdf
```

## Crop pdf

```shell
pdfcrop a.pdf a_cropped.pdf
```


## Zotero Highlight Meanings

- Yellow: highlight
- Red: confusion
- Green: experimental setup
- Purple: acronym

## OpenType Feature Freezer


Example:

```shell
pyftfeatfreeze -f 'c2sc,smcp' -S -U SC -R 'Charis SIL/Charix,CharisSIL/Charix' CharisSIL-R.ttf CharixSC-R.ttf
```

Actual:

```shell
pyftfeatfreeze -f 'ss01,calt' -S -U CALT CascadiaCodeNFItalic.ttf CascadiaCodeNFItalic-CALT.ttf
```

## yabai

```shell
yabai -m query --windows --space 2 | jq '.[].id'
```

## sketchybar-app-font

```shell
pnpm run build:install
```


## Delete .DS_Store Recursively

```shell
find . -name ".DS_Store" -type f -print -delete
```

## Time Machine

## Backup Logs

```shell
printf '\e[3J' && log show --predicate 'subsystem == "com.apple.TimeMachine"' --info --last 12h | grep -F 'eMac' | grep -Fv 'etat' |  grep -Fv 'com.apple.backupd.sandbox.xpc: connection invalid' | awk -F']' '{print substr($0,1,19), $NF}'
```

## OSStatus 80 Error

![link](https://www.reddit.com/r/synology/comments/1c3xoqh/osstatus_error_80_mac_time_machine_synology/)

Delete the Entry "<my-mac's-name-.sparsebundle>" inside the Keychain.

## Shell Redirection

- Redirect stderr through `2>`
    ```shell
    grep fish < /etc/shells > ~/output.txt 2> ~/errors.txt
    ```
- Redirect both stdout and stderr
    ```shell
    make &> make_output.txt
    ```

## LaTeX

### LaTeX Package Update

With tuna's mirror
```shell
sudo tlmgr update --all --repository https://mirrors.tuna.tsinghua.edu.cn/CTAN/systems/texlive/tlnet
```

### `\Omega` Symbol in the `siunitx` Package

`\Omega` symbol not work in `\qty{}{}` command of `siunitx` package.

Need to change `\Omega` to `\ohm`, e.g. `\qty{10}{\Omega}` => `\qty{10}{\ohm}`.

## SSH Test

```shell
ssh -T git@github.com
```

## Get IP Address

```shell
curl ifconfig.me
curl icanhazip.com
```
