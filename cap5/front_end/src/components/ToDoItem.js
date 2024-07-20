import React from 'react';

export const ToDoItem = ({ title, status, send_request }) => {
  const process_status = status === 'PENDING' ? 'edit' : 'delete';

  return (
    <div>
      <p>{title}</p>
      <button
        onClick={() => {
          send_request(title, status);
        }}
      >
        {process_status}
      </button>
    </div>
  );
};
