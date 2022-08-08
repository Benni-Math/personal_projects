/**
 * StudentNodes represent students.
 * @author Benedikt Arnarsson
 */
class StudentNode {
  id: string; // unique, identifiable student ID
  // higher relational weight implies 'better' relation
  relations: Map<string, number>; // StudentNode.id => weight of edge
  // graph: StudentGraph;

  constructor(id: string) {
    this.id = id;
    this.relations = new Map();
  }

  addRelation(student: StudentNode, rel: number) {
    this.relations.set(student.id, rel);
  }

  removeRelation(student: StudentNode):  boolean {
    return this.relations.delete(student.id);
  }


}

/**
 * Weighted undirected graph of students. Weights represent relations.
 * @author Benedikt Arnarsson
 */
export class StudentGraph {
  // StudentNode.id => StudentNode
  students: Map<string, StudentNode> = new Map();

  /**
   * Creates a new StudentNode with set id and adds it to the list of students.
   * If the StudentNode.id is already present, it will return the StudentNode with said ID.
   * @param id ID of the new StudentNode
   * @returns newly created StudentNode
   */
  addStudent(id: string): StudentNode {
    let studentNode = this.students.get(id);

    if (studentNode) return studentNode;

    studentNode = new StudentNode(id);
    this.students.set(id, studentNode);

    return studentNode;
  }

  /**
   * Removes a StudentNode by ID.
   * @param id ID of the StudentNode to be removed
   * @returns the removed StudentNode or undefined if there is no such StudentNode
   */
  removeStudent(id: string): StudentNode | undefined {
    const studentToRemove = this.students.get(id);

    if (!studentToRemove) return undefined;

    this.students.forEach((student) => {
      student.removeRelation(studentToRemove);
    });

    this.students.delete(id);

    return studentToRemove;
  }

  /**
   * Adds a relation between two StudentNodes.
   * Creates the StudentNodes if they do not exist.
   * @param id1 ID of the first student
   * @param id2  ID of the second student
   * @param rel weight of the relation
   */
  addRelation(id1: string, id2: string, rel: number) {
    const student1 = this.addStudent(id1);
    const student2 = this.addStudent(id2);

    student1.addRelation(student2, rel);
    student2.addRelation(student1, rel);
  }

  /**
   * Removes a relation between students.
   * @param id1 ID of the first student
   * @param id2 ID of the second student
   * @return the weight of the relation or undefined if it did not exist
   */
  removeRelation(id1: string, id2: string): number | undefined {
    const student1 = this.students.get(id1);
    const student2 = this.students.get(id2);

    if (student1 && student2) {
      const rel = student1.relations.get(student2.id);
      if (student1.removeRelation(student2) && student2.removeRelation(student1)) {
        return rel;
      }
    }

    return undefined;
  }
};

/**
 * Weight undirected graph of groups. Each node is a list of StudentNodes.
 * Might want to implement a subgraph relation for StudentGraphs...
 * @author Benedikt Arnarsson
 */
class GroupGraph {
  matrix: number[][]; // Useful for summing the 'exterior weight'
  groups: StudentNode[][]; // A StudentNode[] is a group, so this is a list of the groups
  studentGraph: StudentGraph; // The corresponding StudentGraph

  // The idea is that one of these numbers is fixed, which then determines that grouping algorithm used
  groupSize?: number;
  groupNumber?: number;

  // TODO: This function adds a new student in and assigns a group
  addStudent(student: StudentNode): number {
    if (this.groupSize) {
      return this.addStudentFixedGroupSize(student);
    } else {
      return this.addStudentFixedGroupNumber(student);
    }
  }

  addStudentFixedGroupSize(student: StudentNode): number {
    // TODO:num
    let group_num = 0;

    return group_num;
  }

  addStudentFixedGroupNumber(student: StudentNode): number {
    // TODO:
    let group_num = 0;

    return group_num;
  }

  // implements inefficient search for StudentNode...
  // TODO: adding relations
  newRelation(id1: string, id2: string, rel: number): boolean {
    // First check that both of these students exist
    if (!this.studentGraph.students.has(id1) || !this.studentGraph.students.has(id2)) {
      return false;
    }

    let student1 = this.studentGraph.students.get(id1);
    let student2 = this.studentGraph.students.get(id2);

    return true;
  }
  
  // TODO: adding students
  newStudent(id: string, relations?: Map<string, number>, groups?: Array<number>): boolean {
    return true;
  }

  // TODO: constructor StudentGraph -> GroupGraph
  // TODO: create groups / reshuffle / etc (for testing)
  // TODO: from table/DB-entry to GroupGraph
}



export default GroupGraph;