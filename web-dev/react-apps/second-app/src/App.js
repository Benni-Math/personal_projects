import { useState } from 'react'
import Header from './components/Header'
import AddNote from './components/AddNote'
import NoteList from './components/NoteList'
import Subjects from './components/Subjects'
import Button from './components/Button'

function App() {
  const [name, setName] = useState('')
  const [showForm, setShowForm] = useState(false)
  //const [showSubjects, setShowSubjects] = useState(false)
  const [subjects, setSubjects] = useState({1: 'General', 2: 'Test'})

  const [noteList, setNoteList] = useState(
    [{id: 1, subjectID: 1, title: 'Note 1', text: 'test'},
      {id: 2, subjectID: 2, title: 'Another One', text: 'Bites the Dust'}]
  )

  const switchOn = () => {
    setShowForm(!showForm)
    //setShowSubjects(!showSubjects)
  }

  const onAdd = () => {
    console.log('test1')
  }

  return (
    <>
      {name!=='' && <Button 
                      color='grey' 
                      text='Back' 
                      onClick={() => setName('')} 
                    />
      }
      <div className='container'>
        <Header openForm={switchOn} name={name} setName={setName}/>
        {showForm && <AddNote onAdd={onAdd} />}
        {name!=='' && <Subjects 
                        subjects={subjects} 
                        setSubjects={setSubjects}
                        noteList={noteList}
                        setNoteList={setNoteList}
                      />}
      </div>
    </>
  );
}

export default App;
