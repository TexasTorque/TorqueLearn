// This is beutiful JavaScript
function back() {
    window.location.href = (window.location.pathname == "/Tutorials") ? "/" : window.location.pathname.split('/').slice(0, -1).join('/');
}