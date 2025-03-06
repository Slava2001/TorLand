
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
        <ul class="github_link responsive_container" style="margin-left:10px;">
            <a href="https://github.com/Slava2001",  style="margin-right:-25px;">
                <img width="70%" height="70%" src="/svg/github.svg" style="vertical-align:middle">
            </a>
            <a href="https://github.com/Slava2001" class="humble">
            <span style="font-size:160%;">GitHub</span>
            </a>
        </ul>

        <ul>
            <li>
                <button class="outline">
                <a href="/" class="humble">
                    <span lang="en">Main</span>
                    <span lang="ru">Главная</span>
                </a>
                </button>
            </li>
            <li>
              <details class="dropdown" style="margin-top: -6px;">
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
                <div style="display: flex; justify-content: center; flex-direction: column; margin-right: 5px">
                    <button onclick="updateLanguage('ru')" class="secondary language_button" id="ru_flag_btn">
                        <span class="flag-icon flag-icon-ru" id="ru_flag""></span>
                        &nbsp;Русский
                    </button>
                    <button onclick="updateLanguage('en')" class="secondary language_button" id="us_flag_btn">
                        <span class="flag-icon flag-icon-us" id="us_flag"></span>
                        &nbsp;English
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
            <p>E-mail: slavakaplya20011501@gmail.com</p>        
        </section>`
}
