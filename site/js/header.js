
function onLoad(event) {
    setHeader()

    var language = getCookie('language');

    if (language) {
        updateLanguage(language)
    } else {
        var userLang = navigator.language || navigator.userLanguage;
        userLang = userLang.substring(0, 2)

        updateLanguage(userLang)
    }
}

function setHeader() {
    header = document.getElementById("header")
    header.innerHTML = getHeader()
}

function getHeader() {
    return `
    <div class="container">
        <div class="github_link" onclick='location.href="https://github.com/Slava2001"'>
            <div class="github_link_img">
                <img alt="" src="/svg/github.svg">
            </div>
            <div class="github_link_text">
                <p>GitHub</p>
            </div>
        </div>
    
        <div class="menu">
            <ul>
                <li>
                    <a href="/">
                        <span lang="en">Main</span>
                        <span lang="ru">Главная</span>
                    </a>
                </li>
                <li>
                    <a href="/compiler">
                        <span lang="en">Compiler</span>
                        <span lang="ru">Компилятор</span>
                    </a>
                </li>
                <li>
                    <button onclick="updateLanguage('ru')">
                        <span class="flag-icon flag-icon-ru" id="ru_flag"></span>
                        Русский
                    </button>
                    <button onclick="updateLanguage('en')">
                        <span class="flag-icon flag-icon-us" id="us_flag"></span>
                        English
                    </button>
                </li>
            </ul>
        </div>
    </div>
     `
}
