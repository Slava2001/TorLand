const NI_STATEMENTS = {
    'KEYWORDS': [
        'If',
        'Else',
        'Elif',
        'While',
        'Break',
        'Continue',
        'Using',
        'Alias',
        'Scope',
        'Fun',
        'Return',
    ],
    'Dir': ['front', 'frontRight', 'right', 'backRight', 'back', 'backLeft', 'left', 'frontLeft'],
    'Bools': ['False', 'True'],
    'Logic': ['And', 'Or', 'Not'],
    'Builtin': ['dir', 'bot', 'Split', 'Fork', 'Bite', 'ConsumeSunlight', 'AbsorbMinerals', 'IsEmpty', 'IsSibling', 'IsFriend', 'GetLuminosity', 'GetMineralization', 'Sleep', 'Move', 'Face'],
    'Types': ['Int', 'Bool', 'Dir']
};

const NI_KEYWORDS = NI_STATEMENTS.KEYWORDS;
const NI_TYPES = NI_STATEMENTS.Types;
const NI_BOOLS = NI_STATEMENTS.Bools;
const NI_LOGIC = NI_STATEMENTS.Logic;
const NI_BUILTIN = NI_STATEMENTS.Builtin;
const NI_DIRS = NI_STATEMENTS.Dir;

CodeMirror.defineMode("NiLang", function () {
    return {
        startState: function () {
            return {
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

            if (stream.eatWhile(/[a-zA-Z0-9_]/)) {
                word = stream.current().trim();

                const begins_with_number = new RegExp("\\b[0-9]", "g");
                if (word.match(begins_with_number)) {
                    return "number";
                }

                if (word == "Fun") {
                    return "def";
                }

                if (NI_KEYWORDS.includes(word)) {
                    return "keyword";
                }

                if (NI_TYPES.includes(word)) {
                    return "type";
                }
                
                if (NI_LOGIC.includes(word)) {
                    return "operator";
                }

                if (NI_BOOLS.includes(word) || NI_DIRS.includes(word)) {
                    return "builtin";
                }

                const does_it_begin_with_capital_letter = new RegExp("\\b[A-Z].*?\\b", "g")
                if (word.match(does_it_begin_with_capital_letter)) {
                    return "string";
                }

                return null
            }

            let next = stream.next();
            switch (next) {
                case "=":
                case "+":
                case "-":
                case "/":
                case "*":
                case "$":
                case ">":
                case "<":
                  return "operator"
                case ":":
                    let another_one = stream.peek()
                    if (another_one == ":") 
                        {
                            stream.next()
                            return "property"
                        }
              }
            return null;
        }
    };
});

function NiLangHint(cm) {
    const cursor = cm.getCursor();
    const token = cm.getTokenAt(cursor);
    const word = token.string.trim();
    token.type = "error";
    var filtered = [];
    if (word != "") {
        filtered = NI_KEYWORDS.filter(s => (s.toLowerCase().startsWith(word) || s.startsWith(word)) && s != word);
        filtered = filtered.concat(NI_TYPES.filter(s => (s.toLowerCase().startsWith(word) || s.startsWith(word))
            && s != word));
        filtered = filtered.concat(NI_BOOLS.filter(s => (s.toLowerCase().startsWith(word) || s.startsWith(word))
            && s != word));
        filtered = filtered.concat(NI_DIRS.filter(s => (s.toLowerCase().startsWith(word) || s.startsWith(word))
            && s != word));
        filtered = filtered.concat(NI_LOGIC.filter(s => (s.toLowerCase().startsWith(word) || s.startsWith(word))
            && s != word));
        filtered = filtered.concat(NI_BUILTIN.filter(s => (s.toLowerCase().startsWith(word) || s.startsWith(word))
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
