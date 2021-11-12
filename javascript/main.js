/**
 * Renders the to do items from the backend into a HTML div.
 * 
 * @param {Array} items - list of to do items.
 * @param {String} processType - the type of process that the button belonging to the to do item.
 * @param {String} elementId - the id of the HTML element that the items will be inseted.
 * @param {editItem | deleteItem} processFunction - function that is fired once the button is clicked.
 */
function renderItems(items, processType, 
                     elementId, processFunction) {
  
  let placeholder = "<div>";
  let itemsMeta = [];

  for (i = 0; i < items.length; i++) {
    let title = items[i]["title"];
    let placeholderId = 
        processType + "-" + title.replaceAll(" ", "-");
    
    placeholder += 
      '<div class="itemContainer">' + 
        '<p>' + title + '</p>' + 
        '<div class="actionButton" '  + 'id="' + placeholderId + '">' + 
          processType + '</div>' + 
      '</div>';
    
    itemsMeta.push({"id": placeholderId, "title": title});
  }

  placeholder += "</div>";
  document.getElementById(elementId).innerHTML = placeholder;

  for (i = 0; i < itemsMeta.length; i++) {
    document.getElementById(itemsMeta[i]["id"])
            .addEventListener("click", processFunction);
  }
}

/**
 * Packages an API call ready to be sent.
 * 
 * @param {String} url -  the URL endpoint fot the API call.
 * @param {String} method - the method of the API call => POST, GET, PUT
 * @returns {XMLHTTPRequest} - the API packaged API request.
 */
function apiCall(url, method) {
  let xhr = new XMLHttpRequest();
  xhr.withCredentials = true;

  xhr.addEventListener('readystatechange', function() {
    if (this.readyState === this.DONE) {
      renderItems(JSON.parse(this.responseText)["pending_items"], "edit", "pendingItems", editItem);
      renderItems(JSON.parse(this.responseText)["done_items"],  "delete", "doneItems", deleteItem);
      document.getElementById("completeNum").innerHTML = JSON.parse(this.responseText)["done_item_count"];
      document.getElementById("pendingNum").innerHTML = JSON.parse(this.responseText)["pending_item_count"];
    }
  });

  xhr.open(method, url);
  xhr.setRequestHeader('content-type', 'application/json');
  xhr.setRequestHeader('user-token', 'token');
  return xhr;
}

/**
 *  Gets the title from this, and calls the edit API endpoint.
 */
function editItem() {
  let title = this.id.replaceAll("-", " ").replace("edit ", "");
  let call = apiCall("/item/edit", "PUT");
  let json = {
    "title": title,
    "status": "done"
  };

  call.send(JSON.stringify(json));
}

/**
 * Get the title ftom this, and calls the delete API endpoints.
 */
function deleteItem() {
  let title = this.id.replaceAll("-", " ").replace("delete ", "");
  let call = apiCall("/item/delete", "POST");
  let json = {
    "title": title,
    "status": "done"
  };

  call.send(JSON.stringify(json));
}

/**
 * Calls the get items API.
 */
function getItems() {
  let call = apiCall("/item/get", 'GET');
  call.send()
}

getItems();

document.getElementById("create-button")
        .addEventListener("click", createItem);

/**
 * Calls the create items API.
 */
function createItem() {
  let title = document.getElementById("name");
  let call = apiCall("/item/create/" + title.value, "POST");
  call.send();
  document.getElementById("name").value = null;
}