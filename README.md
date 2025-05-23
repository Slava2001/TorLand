### TorLand

TorLand — проект, представляющий собой симуляцию эволюции, где боты, модели примитивных микроорганизмов, развиваются и адаптируются к условиям виртуального мира.

<img src="./bot.png" alt="Image description" width="50" height="50"><br>
Бот - модель примитивного организма. Он выполняет замкнутую последовательность инструкций, которая является его геномом.

<img src="./world.png" alt="Image description" width="200" height="200"><br>
Мир - среда обитания ботов, во многом определяющая пути их эволюции.

Проект состоит из двух основных компонентов:
- botc — компилятор языка [botlang](./botc/bot.md). Он позволяет компилировать и декомпилировать геном ботов.
- torland — мир ботов. Он позволяет выполнять симуляцию их эволюции.

Оба компонента доступны онлайн ([botc](https://wdrop.ru/projects/TorLand/compiler/) и [torland](https://wdrop.ru/projects/TorLand/simulation/)), а также существует возможность собрать офлайн версию:

```
git clone https://github.com/Slava2001/TorLand.git
cd TorLand
cargo build --release

.\target\release\botc

.\target\release\torlandbin
```

### NiLang
Так же вы можете использовать высокоуровневый язык программирвания ботов - NiLang ([ссылка на проект](https://github.com/nikonru/NiLang)).

### Участие:
Мы приветствуем вклад сообщества! Если у вас есть идеи для улучшения, создавайте issue или отправляйте pull request.
