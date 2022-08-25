// This is beautiful JavaScript
// No it's not
// function back() {
//   window.location.href =
//     window.location.pathname == "/Tutorials"
//       ? "/"
//       : window.location.pathname.split("/").slice(0, -1).join("/");
// }
const back = () => window.location.href = window.location.pathname == "/Tutorials" ? "/" : window.location.pathname.split("/").slice(0, -1).join("/");
