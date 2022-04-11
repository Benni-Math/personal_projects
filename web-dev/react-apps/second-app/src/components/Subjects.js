import { useState } from 'react'
import NoteList from './NoteList'
import ButtonList from './ButtonList'

const Subjects = ({ subjects, setSubjects , noteList, setNoteList}) => {
  const [currSubject, setCurrSubject] = useState('Test')

	const onAdd = () => {
		console.log('adding subject...')
	}
	
	return (
    <div>
			<ButtonList 
				subjects={subjects} 
				changeSubj={setCurrSubject} 
				onAdd={onAdd}
			/>

			<NoteList noteList={noteList.filter((note) => subjects[note.subjectID]===currSubject)}/>

		</div>
  )
}

export default Subjects