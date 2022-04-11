import PropTypes from 'prop-types'
import Button from "./Button";

const Header = ({ title, onAdd, showAddTask }) => {
  return (
    <header className="header">
      <h1>{title}</h1>
      {showAddTask ? (
        <Button color="red" text="Close" onClick={onAdd} />
      ) : (
        <Button color="green" text="Add" onClick={onAdd} />
      )}
    </header>
  );
};

Header.defaultProps = {
  title: "My New App",
  showAddTask: false
};

Header.propTypes = {
    title: PropTypes.string.isRequired,
}

// CSS in JS
// const headingStyle = {
//     color: 'white',
//     backgroundColor: 'brown',
// }

export default Header