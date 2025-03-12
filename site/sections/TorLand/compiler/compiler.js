const KEYWORDS = {
    'commands': {
        'nop': [],
        'mov': ['Dir'],
        'rot': ['Dir'],
        'jmp': ['Lable'],
        'cmp': ['Reg', 'Reg'],
        'jme': ['Lable'],
        'jne': ['Lable'],
        'jmg': ['Lable'],
        'jml': ['Lable'],
        'jle': ['Lable'],
        'jge': ['Lable'],
        'jmb': ['Lable'],
        'jnb': ['Lable'],
        'jmc': ['Lable'],
        'jnc': ['Lable'],
        'jmf': ['Lable'],
        'jnf': ['Lable'],
        'chk': ['Dir'],
        'cmpv': ['Reg', 'Val'],
        'split': ['Dir', 'Lable'],
        'fork': ['Dir', 'Lable'],
        'bite': ['Dir'],
        'eatsun': [],
        'absorb': [],
        'call': ['Lable'],
        'ret': [],
        'ld': ['RwReg', 'Reg'],
        'ldv': ['RwReg', 'Val'],
        'ldr': ['Mem', 'Reg'],
        'ldm': ['RwReg', 'Mem'],
    },
    'Dir': {
        'values': ['front', 'frontright', 'right', 'backright', 'back', 'backleft', 'left', 'frontleft'],
        'type': "variable"
    },
    'RwReg': {
        'values': ['Ax', 'Bx', 'Cx', 'Dx'],
        'type': "variable"
    },
    'Reg': {
        'values': ['Ax', 'Bx', 'Cx', 'Dx', 'En', 'Ag', 'Sd', 'Md'],
        'type': "variable"
    },
    'Lable': {
        'parser': LableParser,
        'type': "variable"
    },
    'Val': {
        'parser': ValParser,
        'type': 'number'
    },
    'Mem': {
        'parser': MemParser,
        'type': 'number'
    }
};
const COMMANDS = Object.keys(KEYWORDS.commands);
const REGS = KEYWORDS.Reg.values;
const DIRS = KEYWORDS.Dir.values;
const LABLE_REGEX = /^[A-Za-z_]+[A-Za-z0-9_]*:$/;


function ValParser(stream, state) {
    if (/^[+-]?[0-9]*$/.test(stream.current())) {
        return KEYWORDS['Val'].type;
    }
    return 'error';
}

function MemParser(stream, state) {
    if (/^[[]{1}[0-9]+[]]{1}$/.test(stream.current())) {
        return KEYWORDS['Mem'].type;
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
                    if (KEYWORDS[exp_ty].values.some(x => x.toLowerCase() == word)) {
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

function get_lables() {
    let all_code = editor.getValue();
    let words = all_code.split(/\s/);
    let _lables = words.filter(s => LABLE_REGEX.test(s));
    let lables = _lables.map(s => s.replace(':', ''));
    return lables;
}

function BotLangHint(cm) {
    const cursor = cm.getCursor();
    const token = cm.getTokenAt(cursor);
    const word = token.string.trim().toLowerCase();
    token.type = "error";
    var filtered = [];
    if (word != "") {
        filtered = COMMANDS.filter(s => s.startsWith(word) && s != word);
        filtered = filtered.concat(REGS.filter(s => s.toLowerCase().startsWith(word)
            && s.toLowerCase() != word));
        filtered = filtered.concat(DIRS.filter(s => s.toLowerCase().startsWith(word)
            && s.toLowerCase() != word));
        filtered = filtered.concat(get_lables().filter(s => s.toLowerCase().startsWith(word)
            && s.toLowerCase() != word));
    }

    return {
        list: filtered,
        from: { line: cursor.line, ch: token.start },
        to: { line: cursor.line, ch: cursor.ch }
    };
}

function botlang_init() {
    const editor = CodeMirror.fromTextArea(document.getElementById('input'), {
        lineNumbers: true,
        mode: 'BotLang',
        theme: 'dracula',
        height: 'auto', 
        lineWrapping: true,
    });

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

    return editor
}
