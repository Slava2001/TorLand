# Бот

Бот - модель примитивного организма

Вид - совокупность ботов одного цвета

Колония - объединение ботов одного вида. Возможно существование нескольких колоний одного вида

## Поведение бота

Поведение бота описано в его геноме, который представляет собой замкнутую последовательность инструкций, то есть когда бот доходит до последней, переходит к первой.

## Генетический код

    todo

## Состояние Бота

Состояние бота описывается следующими параметрами:

- значения регистров и флагов
- направление взгляда
- счетчик команд
- стек адресов возврат

## Язык программирования ботов botlang

botlang - язык описания поведения ботов.

### Регистры

- AX, BX, CX, DX - регистры общего назначения. Доступно чтение и запись
- EN - текущие количество энергии. Доступно только чтение
- AG - возраст бота в циклах. Доступно только чтение
- SD - разница в уровне освещенности между текущей и исследуемой клеткой. Доступно только чтение
- MD - разница в уровне минерализации между текущей и исследуемой клеткой. Доступно только чтение

### Флаги

- FS - флаг разницы
- FZ - флаг равенства
- EF - исследуемая клетка свободна
- EB - исследуемая клетка занята ботом того же вида
- EС - исследуемая клетка занята ботом той же колони

### Команды



|команда|описание|
|-|-|
| **nop** | команда пропуска цикла |
| **mov&nbsp;[dir]** | команда перемещения. Бот перемещается в указанном направлении на 1 шаг |
| **rot&nbsp;[dir]** | команда поворота. Бот поворачивается в указанном направлении |
| **jmp&nbsp;[Lable]** | команда безусловного перехода |
| **jmg/jnl&nbsp;[Lable]** | команда условного перехода. Переход происходит если установлен / не установлен флаг `FS` |
| **jme/jne&nbsp;[Lable]** | команда условного перехода. Переход происходит если установлен / не установлен флаг `FZ` |
| **jmf/jnf&nbsp;[Lable]** | команда условного перехода. Переход происходит если установлен / не установлен флаг `EF` |
| **jmb/jnb&nbsp;[Lable]** | команда условного перехода. Переход происходит если установлен / не установлен флаг `EB` |
| **jmc/jnc&nbsp;[Lable]** | команда условного перехода. Переход происходит если установлен / не установлен флаг `EС` |
| **jge/jle&nbsp;[Lable]** | команда условного перехода. Переход происходит если установлен флаг `FZ` или установлен / не установлен флаг `FS` |
| **chk&nbsp;[dir]** | команда поверки окружения. Бот исследует клетку в указанном направлении. Команда изменяет регистры `SD` и `MD` и флаги `EF`, `EB` и `EС` |
| **cmp&nbsp;[reg]&nbsp;[reg]** | команда выполнения проверки. <br> cmp a b <br> a &ge; b => `FS` = 1 <br> a < b => `FS` = 0 <br> a == b => `FZ` = 1 <br> a != b => `FZ` = 0  |
| **cmpv&nbsp;[reg]&nbsp;[val]** | аналогична `cmp` |
| **split&nbsp;[dir]&nbsp;[Lable]** | команда деления. Бот, если у него достаточно энергии, создает свою копию в указанном направлении. Копия начинает выполнение кода с указанной метки. Копия наследует направление взгляда |
| **forc&nbsp;[dir]&nbsp;[Lable]** | аналогична команде `forc`, но при копировании с небольшим шансом может возникнуть мутация. Также новый бот создает собственную колонию |
| **bite&nbsp;[dir]** | команда атаки. Бот атакует в указанном направлении, если в выбранном направлении находится другой бот, атакующий забирает у жертвы часть энергии |
| **eatsun** | команда выполнения фотосинтеза. Бот получает энергию от поглощения солнечного света. Количество энергии зависит от освещенности |
| **absorb** | команда поглощения минералов. Бот погашает минералы из почвы |
| **call&nbsp;[Lable]** | команда безусловного перехода с сохранением адреса следующей команды в стек адресов возврата |
| **ret** | команда безусловного перехода на адрес, извлечённый из стека адресов возврата |
| **ld&nbsp;[reg1]&nbsp;[reg2]** | команда сохранения значения регистра `reg2` в регистр `reg1`|
| **ldv&nbsp;[reg]&nbsp;[val]** | аналогична `ld` |

- [Lable] - метка. Используется для указания места для перехода
- [val] - целочисленная константа
- [dir] - направление бота
- цикл - одна итерация обновления мира

## Компилятор генетического кода


Передача энергии и минералов

### Дробные переходы

Предположим что геном бота описан следующим кодом:

```
start:
    mov up
    jmp start
```

если при компиляции последовательно укладывать команды и их аргументы, геном выгляди так:

|0|mov|
|-|-|
|**1**|**up**|
|**2**|**jmp**|
|**3**|**0**|

Из-за мутации геном может приобрести следующий вид:

|0|mov|
|-|-|
|**1**|**up**|
|**2**|**jmp**|
|**3**|**1**|

(изменился аргумент команды `jmp` (3я ячейка))

Теперь при достижении 2й ячейки переход будет выполнен не на команду `mov`, а на ее аргумент, то есть `up`. Это означает, что `up` будет интерпретирована как команда, а `jmp` как её аргумент.

Этот эффект позволит ботам создавать эффективно упакованный код, то есть часть генома может описывать разное поведение в зависимости от смещения. Но с другой стороны это в значительной степени усложняет реверс инжиниринг генома созданного путем эволюции.

Для устранения данной проблемы было принято решение сделать команды атомарными, то есть упаковывать аргументы команды в ту же ячейку что и саму команду. То есть геном будет выглядеть следующим образом:

|0|mov up|
|-|-|
|**1**|**jmp 0**|

Таким образом любой переход всегда будет осуществляться на команду и никогда на ее аргумент, что упростит автоматизацию трансляции генома в код.
