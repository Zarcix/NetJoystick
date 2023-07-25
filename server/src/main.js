const { listen } = window.__TAURI__.event;

const { invoke } = window.__TAURI__.tauri;

const unlisten = listen('reload_clients', (_) => {
  invoke('reload_clients').then((response) => {
    
    // Connected Clients
    var connected_client_table = document.getElementById("connectedClients");
    connected_client_table.innerHTML = ""
    for (client of response[0]) {
      var ip = client.split(":")[0];
      connected_client_table.innerHTML = connected_client_table.innerHTML + "<tr id=\"client\"><td id=\"ip\" class=\"clitabledata\">" + ip + "<td id=\"buttons\" class=\"clitabledata\"><button onclick=\"invoke('disconnect_client', {client: '" + client + "'})\">disconnect</button></td></tr>"
    }

    // Potential Clients

    var pending_client_table = document.getElementById("pendingClients");
    pending_client_table.innerHTML = ""
    for (client of response[1]) {
      var ip = client.split(":")[0];
      pending_client_table.innerHTML = pending_client_table.innerHTML + "<tr><td class='clitabledata'> " + ip + " <button onclick=\"invoke('accept_client', {client: '" + client + "'})\">accept</button><button onclick=\"invoke('deny_client', {client: '" + client + "'})\">deny</button></td></tr>"
    }
    
  })
})

function parent(elem) {
  alert(elem.parentNode.parentNode)
}

function switchPages(show, hide) {
  document.getElementById(show).style.display = "block";
  document.getElementById(hide).style.display = "none";
}


