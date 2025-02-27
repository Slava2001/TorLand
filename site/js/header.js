function getHeader(){
    return `
<div class="container">
    <div class="github_link" onclick='location.href="https://github.com/Slava2001"'>
        <div class="github_link_img">
            <img alt="" src="https://avatars.githubusercontent.com/u/45189467?v=4">
        </div>
        <div class="github_link_text">
            <p>GitHub/Slava</p>
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

function setHeader(){
    header = document.getElementById("header")
    header.innerHTML = getHeader()
}