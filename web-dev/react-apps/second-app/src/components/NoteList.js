import Note from './Note'

const NoteList = ({ noteList }) => {
  return (
    <div>
      {noteList.map((text) => <Note noteInput={text}/>)} 
    </div>
  )
}

export default NoteList