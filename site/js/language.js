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

        setCookie('language', "ru", 7);

        colerId = "coler_ru"
        actionId = "action_ru"
        startLabel = "Старт"
        stopLabel = "Стоп"
    } else {
        selectLanguage("en")

        setFlagGrey("ru_flag")
        setFlagColorful("us_flag")

        setCookie('language', "en", 7);

        colerId = "coler_en"
        actionId = "action_en"
        startLabel = "Start"
        stopLabel = "Stop"
    }
};

function setFlagGrey(flag) {
    element = document.getElementById(flag)
    element.style.filter = "grayscale(100%)"
}

function setFlagColorful(flag) {
    element = document.getElementById(flag)
    element.style.filter = "grayscale(0%)"
}
