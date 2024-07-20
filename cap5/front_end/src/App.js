import { useEffect, useState } from 'react';

import './App.css';
import { ToDoItem } from './components/ToDoItem';
import { CreateToDoItem } from './components/CreateToDoItem';

function App() {
  const [pending_items, set_pending_items] = useState([]);
  const [done_items, set_done_items] = useState([]);
  const [pending_items_count, set_pending_items_count] = useState(0);
  const [done_items_count, set_done_items_count] = useState(0);

  useEffect(() => {
    get_items();
  }, []);

  const get_items = () => {
    fetch('http://192.168.31.197:8080/v1/item/get')
      .then((response) => response.json())
      .then(update_state);
  };

  const send_request = (title, status, action) => {
    fetch(`http://192.168.31.197:8080/v1/item/${action}`, {
      method: 'POST',
      headers: { 'user-token': 'token', 'Content-Type': 'application/json' },
      body: JSON.stringify({ title, status: inverse_status(status) }),
    })
      .then((response) => response.json())
      .then(update_state);
  };

  const inverse_status = (status) => {
    if (status === 'PENDING') {
      return 'DONE';
    }

    return 'PENDING';
  };

  const update_state = (response) => {
    if (pending_items !== response.pending_items) {
      set_pending_items(response.pending_items);
    }

    if (done_items !== response.done_items) {
      set_done_items(response.done_items);
    }

    if (pending_items_count !== response.done_items) {
      set_pending_items_count(response.done_items);
    }

    if (done_items_count !== response.done_items) {
      set_done_items_count(response.done_items);
    }
  };

  return (
    <div className="main-container">
      <h1>Items</h1>

      <h2>Done Items</h2>
      <ul id="done-items">
        {done_items.map(({ title, status }) => (
          <ToDoItem
            key={title}
            status={status}
            title={title}
            send_request={send_request}
          />
        ))}
      </ul>

      <h2>To Do Items</h2>
      <ul id="pending-items">
        {pending_items.map(({ title, status }) => (
          <ToDoItem
            key={title}
            status={status}
            title={title}
            send_request={send_request}
          />
        ))}
      </ul>

      <CreateToDoItem />
    </div>
  );
}

export default App;
