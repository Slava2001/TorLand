const NI_KEYWORDS = {
    'NI_COMMANDS': {
        'If': [],
        'While': [],
        'Break': [],
        'Continue': [],
        'Using': [],
        'Else': [],
        'Elif': [],
        'And': [],
        'Or': [],
        'Fun': [],
        '$': [],
        'Return': [],
    },
    'Dir': {
        'values': ['front', 'frontright', 'right', 'backright', 'back', 'backleft', 'left', 'frontleft'],
        'type': "variable"
    },
    'Bools': {
        'values': ['False', 'True'],
        'type': "variable"
    },
    'Types': {
        'values': ['Int', 'Bool', 'Dir'],
        'type': "variable"
    },
};
const NI_COMMANDS = Object.keys(NI_KEYWORDS.NI_COMMANDS);
const NI_TYPES = NI_KEYWORDS.Types.values;
const NI_BOOLS = NI_KEYWORDS.Bools.values;
const NI_DIRS = NI_KEYWORDS.Dir.values;

function ni_ValParser(stream, state) {
    if (/^[+-]?[0-9]*$/.test(stream.current())) {
        return NI_KEYWORDS['Val'].type;
    }
    return 'error';
}

function ni_MemParser(stream, state) {
    if (/^[[]{1}[0-9]+[]]{1}$/.test(stream.current())) {
        return NI_KEYWORDS['Mem'].type;
    }
    return 'error';
}

function ni_LableParser(stream, state) {
    if (/^[A-Za-z_]+[A-Za-z0-9_]*$/.test(stream.current())) {
        return NI_KEYWORDS['Lable'].type;
    }
    return 'error';
}

CodeMirror.defineMode("NiLang", function () {
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
            if (stream.match("#")) {
                stream.skipToEnd();
                return "comment";
            }

            if (stream.eatWhile(/[^\s]/)) {
                word = stream.current().trim();

                // // Args
                // if (state.expect_args.length > 0) {
                //     exp_ty = state.expect_args.shift();
                //     if (NI_KEYWORDS[exp_ty].parser) {
                //         return NI_KEYWORDS[exp_ty].parser(stream, state);
                //     }
                //     if (NI_KEYWORDS[exp_ty].values.some(x => x == word)) {
                //         return NI_KEYWORDS[exp_ty].type;
                //     }
                //     return "error ";
                // }

                // // Lable
                // if (LABLE_REGEX.test(word)) {
                //     if (state.lables.includes(word.slice(0, -1))) {
                //         return "error";
                //     }
                //     return "def";
                // }

                // NI_COMMANDS
                if (NI_COMMANDS.includes(word)) {
                    state.expect_args = [...NI_KEYWORDS.NI_COMMANDS[word]];
                    return "keyword";
                }
            }

            stream.next();
            return null;
        }
    };
});

function ni_get_lables() {
    let all_code = nilang_editor.getValue();
    let words = all_code.split(/\s/);
    let _lables = words.filter(s => LABLE_REGEX.test(s));
    let lables = _lables.map(s => s.replace(':', ''));
    return lables;
}

function NiLangHint(cm) {
    const cursor = cm.getCursor();
    const token = cm.getTokenAt(cursor);
    const word = token.string.trim();
    token.type = "error";
    var filtered = [];
    if (word != "") {
        filtered = NI_COMMANDS.filter(s => s.startsWith(word) && s != word);
        filtered = filtered.concat(NI_TYPES.filter(s => s.startsWith(word)
            && s != word));
        filtered = filtered.concat(NI_BOOLS.filter(s => s.startsWith(word)
            && s != word));
        filtered = filtered.concat(NI_DIRS.filter(s => s.startsWith(word)
            && s != word));
        filtered = filtered.concat(ni_get_lables().filter(s => s.startsWith(word)
            && s != word));
    }

    return {
        list: filtered,
        from: { line: cursor.line, ch: token.start },
        to: { line: cursor.line, ch: cursor.ch }
    };
}

function nilang_init() {
    const nilang_editor = CodeMirror.fromTextArea(document.getElementById('nilang_input'), {
        lineNumbers: true,
        mode: 'NiLang',
        theme: 'dracula',
        height: 'auto',
        lineWrapping: true,
    });

    nilang_editor.addKeyMap({
        'Tab': function (cm) {
            const cursor = cm.getCursor();
            const suggestions = NiLangHint(cm).list;
            if (suggestions.length > 0) {
                cm.showHint({ hint: NiLangHint });
            } else {
                cm.replaceRange('    ', cursor);
            }
        }
    });

    nilang_editor.on('inputRead', function () {
        nilang_editor.showHint({ hint: NiLangHint, completeSingle: false });
    });

    return nilang_editor
}
