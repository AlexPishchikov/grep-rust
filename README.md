#### grep

Ищет PATTERN в файлах FILES и выводит в консоль результат, соответствующий OPTIONS.

Если не заданы параметры, выводятся все строки в файлах, содержащие PATTERN.

Если не указаны файлы, будет производиться чтение из stdin.

#### Compile

`rustc main.rs -o grep`

#### Syntax

`./grep [OPTIONS] PATTERN [FILES]`

#### OPTIONS

- `-v` - поиск строк НЕ содержащих PATTERN;

- `-H` - выводить имя файла вместе со строкой результата. Работает по умолчанию, если было передано больше одного файла;

- `-h` - НЕ выводить имя файла вместе со строкой результата. Работает по умолчанию, если был передан один файл;

- `-c` - вместо строки выводить кол-во совпадений на каждый файл;

- `-m NUM` - остановить поиск после NUM совпадений.
