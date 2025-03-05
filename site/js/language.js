var colerId = "coler_en"
var actionId = "action_en"
var startLabel = "Start"
var stopLabel = "Stop"

function selectLanguage(language) {
    $("[lang]").each(function () {
        if ($(this).attr("lang") == language)
            $(this).show();
        else
            $(this).hide();
    });
}

function updateLanguage(language) {
    languageSelector = document.getElementById("language_selector")

    if (language == "ru") {
        selectLanguage("ru")

        setFlagGrey("us_flag")
        setFlagColorful("ru_flag")

        setButtonContrast("us_flag_btn", false)
        setButtonContrast("ru_flag_btn", true)

        setCookie('language', "ru", 7);

        colerId = "coler_ru"
        actionId = "action_ru"
        startLabel = "Старт"
        stopLabel = "Стоп"
    } else {
        selectLanguage("en")

        setFlagGrey("ru_flag")
        setFlagColorful("us_flag")

        setButtonContrast("ru_flag_btn", false)
        setButtonContrast("us_flag_btn", true)

        setCookie('language', "en", 7);

        colerId = "coler_en"
        actionId = "action_en"
        startLabel = "Start"
        stopLabel = "Stop"
    }
};

function setFlagGrey(flag) {
    element = document.getElementById(flag)
    element.style.filter = "grayscale(60%)"
}

function setFlagColorful(flag) {
    element = document.getElementById(flag)
    element.style.filter = "grayscale(0%)"
}

function setButtonContrast(id, is_contrast) {
    cls = "outline"
    element = document.getElementById(id)
    if (is_contrast) {
        element.classList.add(cls)
    } else if (element.classList.contains(cls)) {
        element.classList.remove(cls)
    }
}
