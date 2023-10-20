let id = window.location.pathname.split("/").pop();
let disabled = false;

function trigger() {
  if (disabled) {
    return;
  }
  send(`/api/triggers/v1/use/${id}`);
}

function revoke() {
  if (disabled) {
    return;
  }
  send(`/api/triggers/v1/revoke/${id}`);
}

function send(url) {
  fetch(url)
    .then((resp) => resp.json())
    .then((res) => {
      switch (res.type) {
        case "triggered":
          display("Trigger ran successfully.");
          disabled = true;
          break;
        case "revoked":
          display("Trigger revoked successfully.");
          disabled = true;
          break;
        case "error":
          display(`API returned with error: ${JSON.stringify(e)}`);
          break;
        default:
          display(`Unexpected response: ${JSON.stringify(e)}`);
      }
    })
    .catch((e) => {
      display(`An error occured: ${JSON.stringify(e)}`);
    });
}

function display(text) {
  document.getElementById("output").innerText = text;
}
