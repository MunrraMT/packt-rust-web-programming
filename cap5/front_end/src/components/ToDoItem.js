import React from 'react';

export const ToDoItem = ({ title, status, send_request }) => {
  const process_status = status === 'PENDING' ? 'edit' : 'delete';

  return (
    <div className="item-container">
      <p>{title}</p>
      <button
        className="action-button"
        onClick={() => {
          send_request(title, status, process_status);
        }}
      >
        {process_status}
      </button>
    </div>
  );
};
