
function onLoad(event) {
    initPage()

    var language = getCookie('language');

    if (language) {
        updateLanguage(language)
    } else {
        var userLang = navigator.language || navigator.userLanguage;
        userLang = userLang.substring(0, 2)

        updateLanguage(userLang)
    }
}

function initPage() {
    header = document.getElementById("header")
    header.innerHTML = getHeader()

    footer = document.getElementById("footer")
    footer.innerHTML = getFooter()
}

function getHeader() {
    return `
    <nav>
        <ul class="github_link">
            <a href="https://github.com/Slava2001">
                <img alt="" src="/svg/github.svg">
            </a>
            <p>GitHub</p>
        </ul>

        <ul>
            <li>
                <button>
                <a href="/">
                    <span lang="en">Main</span>
                    <span lang="ru">Главная</span>
                </a>
                </button>
            </li>
            <li>
              <details class="dropdown" style="margin-top: -5px;">
                <summary>
                  TorLand
                </summary>
                <ul dir="rtl">
                  <li>
                      <a href="/sections/TorLand/simulation/index.html">
                          <span lang="en">Simulation</span>
                          <span lang="ru">Симуляция</span>
                      </a>
                  </li>
                  <li>
                      <a href="/sections/TorLand/compiler/index.html">
                          <span lang="en">Compiler</span>
                          <span lang="ru">Компилятор</span>
                      </a>
                  </li>
                </ul>
              </details>
            </li>
            <li>
                <div style="display: flex; justify-content: center; flex-direction: column; font-size: 60%">
                    <button onclick="updateLanguage('ru')" class="secondary" style="height: 25px; font-size: 90%; text-align: center;">
                        <span class="flag-icon flag-icon-ru" id="ru_flag""></span>
                        Русский
                    </button>
                    <button onclick="updateLanguage('en')" class="secondary" style="height: 25px; font-size: 90%; text-align: center;">
                        <span class="flag-icon flag-icon-us" id="us_flag"></span>
                        English
                    </button>
                </div>
            </li>
        </ul>
    </nav>
     `
}

function getFooter() {
    return `
        <section style="display: flex; justify-content: center;">
            <p>email: slavakaplya20011501@gmail.com</p>        
        </section>`
}
