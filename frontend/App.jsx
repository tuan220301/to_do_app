import 'regenerator-runtime/runtime';
import React from 'react';

import './assets/global.css';

// import { getGreetingFromContract, setGreetingOnContract } from './near-api';
import { addTask, deleteTask, getTask } from './near-api';
import { EducationalText, SignInPrompt, SignOutButton } from './ui-components';

import React, { useState } from "react";

export default function App() {
  const [valueFromBlockchain, setValueFromBlockchain] = React.useState();
  const [tasks, setTasks] = useState([]);
  const [uiPleaseWait, setUiPleaseWait] = React.useState(true);
  // Get blockchian state once on component load
  
  React.useEffect(() => {
    getTask()
      .then((res) => setTasks(res))
      .catch((err) => console.error(err))
      .finally(() => {
        setUiPleaseWait(false);
      });
  }, []);
  // let tasks = [];
  // tasks.push(getTask);
  // console.log("tasks 2: " + JSON.stringify(setTasks))
  /// If user not signed-in with wallet - show prompt
  if (!window.walletConnection.isSignedIn()) {
    // Sign-in flow will reload the page later
    return <SignInPrompt greeting={valueFromBlockchain}/>;
  }

  const [taskValue, setTaskValue] = useState("");
  const handleAddTask = event  => {
    setTaskValue(event.target.value);
  }
  const handleClick = event => {
    event.preventDefault();
    // console.log('handleClick 👉️', taskValue);
    addTask(taskValue).then(() => {
      getTasks()
        .then((res) => setTasks(res))
        .finally(() => {
          setTaskValue("");
        });
    });
  };
  // const deleteTaskButton = (id) =>{
  //   console.log('id: ')
  //   let taskDeleteSelected = tasks.filter((task) => task.id === id)[0]
  //   console.log('taskDeleteSelected: ' + taskDeleteSelected.id)
  //   // deleteTask(deleteTaskButton.id)
  // }
  function deleteTaskBtn(id){
    deleteTask(id).then(()=>{
      getTasks()
        .then((res) => setTasks(res))
        .finally(() => {
          setTaskValue("");
        });
    })
  }
  const stye_task ={
    // textAlign: "center",
    color: "white",
    listStyleType: "none"
  };
  const button_style ={
    float: "right",
    backgroundColor: "green"
  }
  return (
    <>
      <SignOutButton accountId={window.accountId}/>
      
      <main className={uiPleaseWait ? 'please-wait' : ''}>
        <h1>
          Tasks today: 
        </h1>
        <div className="greeting">
          <ul className='center' style={stye_task}>
            {
              tasks.map(task =>(
                <li key={task.id}>
                  {task.content}
                  <button style={button_style} onClick={() => deleteTaskBtn(task.id)}>delete</button>
                </li>
              ))
            }
          </ul>
        </div>

        {/* <form onSubmit={handleAddTask} className="change"> */}
          <div className="change">
            <label>Change greeting:</label>
            <div>
              <input
                autoComplete="off"
                // defaultValue={valueFromBlockchain}
                // value={taskValue}
                value={taskValue}
                onChange={handleAddTask}
                id="greetingInput"
                
              />
              <button onClick={handleClick}>
                <span>Save</span>
                <div className="loader"></div>
              </button>
            </div>
          </div>
        {/* </form> */}
        {/* <EducationalText/> */}
      </main>
    </>
  );
}
