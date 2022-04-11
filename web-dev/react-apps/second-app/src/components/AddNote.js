import { useState } from 'react'

const AddNote = ({ onAdd }) => {
	const [title, setTitle] = useState('')
	const [text, setText] = useState('')

	const onSubmit = (e) => {
		e.preventDefault()

		if (!text || !title) {
			alert('Please finish your entry')
			return
		}

		onAdd({title, text})

		setTitle('')
		setText('')
	}

	return (
		<form className='add-form' onSumbit={onSubmit}>
			<div classname='form-control'>
				<label>Title</label>
				<input 
					type='text' 
					placeholder='Add Title'
					value={title}
					onChange={(e) => setTitle(e.target.value)}
				/>
			</div>
			<div classname='form-control'>
			<label>Text</label>
			<input 
				type='text' 
				placeholder='Add Text'
				value={text}
				onChange={(e) => setText(e.target.value)}
			/>
			</div>
		</form>
	)
}

export default AddNote