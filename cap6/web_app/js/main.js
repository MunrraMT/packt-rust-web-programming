function render_items(items, process_type, element_id, process_function) {
  const container = document.createElement('div');
  const container_parent = document.querySelector(`#${element_id}`);

  items.forEach((item) => {
    const title_slug = item.title.replace(/ /g, '-');
    const id = process_type + '-' + title_slug;
    const div = document.createElement('div');
    const text = document.createElement('p');
    const btn = document.createElement('button');

    text.textContent = item.title;

    btn.id = id;
    btn.className = 'action-button';
    btn.textContent = 'edit';
    btn.addEventListener('click', process_function);

    div.className = 'item-container';
    div.appendChild(text);
    div.appendChild(btn);

    container.appendChild(div);
  });

  container_parent.innerHTML = '';
  container_parent.appendChild(container);
}

function api_call(url, method = 'GET', body = '') {
  const settings = {
    method,
    headers: { 'user-token': 'token', 'Content-Type': 'application/json' },
  };

  if (method === 'POST') {
    settings.body = JSON.stringify(body);
  }

  fetch(url, settings)
    .then((response) => response.json())
    .then((response) => {
      render_items(response.pending_items, 'edit', 'pending-items', edit_item);

      render_items(response.done_items, 'delete', 'done-items', delete_item);

      change_header_values(
        response.pending_items_count,
        response.done_items_count,
      );
    });
}

function change_header_values(pending, done) {
  document.querySelector('#done-num').textContent = done;
  document.querySelector('#pending-num').textContent = pending;
}

function edit_item(e) {
  const title = e.target.id.replace(/[-]/g, ' ').replace('edit ', '');
  const body = { title, status: 'DONE' };

  console.log({ body });

  api_call('/v1/item/edit', 'POST', body);
}

function delete_item(e) {
  const title = e.target.id.replace(/[-]/g, ' ').replace('delete ', '');
  const body = { title, status: 'DONE' };

  api_call('/v1/item/delete', 'POST', body);
}

function create_item() {
  const title_input = document.querySelector('#name');

  api_call(`/v1/item/create/${title_input.value}`, 'POST');

  title_input.value = '';
}

function get_items() {
  api_call('/v1/item/get');
}

get_items();

document.querySelector('#create-button').addEventListener('click', create_item);