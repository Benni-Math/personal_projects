import "./styles.css";
import Student from "./components/Student";
import StudentList from "./components/StudentList";

export default function App() {
  const student1 = {
    id: 1,
    image:
      "https://www.compass-insurance-agency.com/wp-content/uploads/2016/09/girl-education-980x653.jpg",
    firstName: "Velos",
    lastName: "Erpator",
    email: "velos@pangea.edu",
    hobby: "Running"
  };

  const student2 = {
    id: 2,
    image: "http://www.ccdaily.com/wp-content/uploads/2019/01/studentA.jpg",
    firstName: "Kah",
    lastName: "Nohli",
    email: "kah@northend.edu",
    hobby: "Cooking"
  };

  const students = [student1, student2];

  return (
    <div className="App">
      <h1>Student Profiles</h1>
      {/* <Student studentInfo={student} /> */}
      <StudentList students={students} />
    </div>
  );
}
