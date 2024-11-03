const KEYWORDS = {
    'commands': {
        'nop': [],
        'mov': ['Dir'],
        'rot': ['Dir'],
        'jmp': ['Lable'],
        'jmg': ['Lable'],
        'jnl': ['Lable'],
        'jme': ['Lable'],
        'jne': ['Lable'],
        'jmf': ['Lable'],
        'jnf': ['Lable'],
        'jmb': ['Lable'],
        'jnb': ['Lable'],
        'jmc': ['Lable'],
        'jnc': ['Lable'],
        'jge': ['Lable'],
        'jle': ['Lable'],
        'chk': ['Dir'],
        'cmp': ['Reg', 'Reg'],
        'cmpv': ['Reg', 'Val'],
        'split': ['Dir', 'Lable'],
        'forc': ['Dir', 'Lable'],
        'bite': ['Dir'],
        'eatsun': [],
        'absorb': [],
        'call': ['Lable'],
        'ret': [],
        'load': ['Reg', 'Reg'],
        'loadv': ['RwReg', 'Val']
    },
    'Dir': {
        'values': ['front', 'frontright', 'right', 'backright', 'back', 'backleft', 'left', 'frontleft'],
        'type': "variable"
    },
    'RwReg': {
        'values': ['ax', 'bx', 'cx', 'dx'],
        'type': "variable"
    },
    'Reg': {
        'values': ['ax', 'bx', 'cx', 'dx', 'en', 'ag', 'sd', 'md'],
        'type': "variable"
    },
    'Lable': {
        'parser': LableParser,
        'type': "variable"
    },
    'Val': {
        'parser': ValParser,
        'type': 'number'
    }
};
const COMMANDS = Object.keys(KEYWORDS.commands);
const LABLE_REGEX = /^[A-Za-z_]+[A-Za-z0-9_]*:$/;


function ValParser(stream, state) {
    if (/^[+-]?[0-9]*$/.test(stream.current())) {
        return KEYWORDS['Val'].type;
    }
    return 'error';
}

function LableParser(stream, state) {
    if (/^[A-Za-z_]+[A-Za-z0-9_]*$/.test(stream.current())) {
        return KEYWORDS['Lable'].type;
    }
    return 'error';
}

CodeMirror.defineMode("BotLang", function () {
    return {
        startState: function () {
            return {
                lables: [],
                expect_args: []
            };
        },
        token: function (stream, state) {
            if (stream.eatSpace()) {
                return null;
            }
            if (stream.match("//")) {
                stream.skipToEnd();
                return "comment";
            }

            if (stream.eatWhile(/[^\s]/)) {
                word = stream.current().trim().toLowerCase();

                // Args
                if (state.expect_args.length > 0) {
                    exp_ty = state.expect_args.shift();
                    if (KEYWORDS[exp_ty].parser) {
                        return KEYWORDS[exp_ty].parser(stream, state);
                    }
                    if (KEYWORDS[exp_ty].values.includes(word)) {

                        return KEYWORDS[exp_ty].type;
                    }
                    return "error ";
                }

                // Lable
                if (LABLE_REGEX.test(word)) {
                    if (state.lables.includes(word.slice(0, -1))) {
                        return "error";
                    }
                    return "def";
                }

                // Commands
                if (COMMANDS.includes(word)) {
                    state.expect_args = [...KEYWORDS.commands[word]];
                    return "keyword";
                }
            }

            stream.next();
            return null;
        }
    };
});

function BotLangHint(cm) {
    const cursor = cm.getCursor();
    const token = cm.getTokenAt(cursor);
    const word = token.string.trim()
    token.type = "error";
    var filtered = [];
    if (word != "") {
        filtered = COMMANDS.filter(s => s.startsWith(word) && s != word);
    }

    return {
        list: filtered,
        from: { line: cursor.line, ch: token.start },
        to: { line: cursor.line, ch: cursor.ch }
    };
}

const editor = CodeMirror.fromTextArea(document.getElementById('input'), {
    lineNumbers: true,
    mode: 'BotLang',
    theme: 'dracula',
    lineWrapping: true,
});
editor.setSize("100%", "100%");

editor.addKeyMap({
    'Tab': function (cm) {
        const cursor = cm.getCursor();
        const suggestions = BotLangHint(cm).list;
        if (suggestions.length > 0) {
            cm.showHint({ hint: BotLangHint });
        } else {
            cm.replaceRange('    ', cursor);
        }
    }
});

editor.on('inputRead', function () {
    editor.showHint({ hint: BotLangHint, completeSingle: false });
});