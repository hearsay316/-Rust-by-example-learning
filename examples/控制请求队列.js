class Scheduler {
    constructor() {
        this.task = []
        this.isRun = false
    }
    add(fn) {
        this.task.push(fn)
        !this.isRun&&this.run(2)
    }
    run(count) {
        if (this.task.length >= count) {
            const task = this.task.splice(0, count)
            this.isRun = true
            Promise.all(task.map((fn) => fn())).then(() => {
                this.isRun = false
                this.run(count)
            })
        }
    }
}
const timeout = (time) => new Promise(resolve => {
    setTimeout(resolve, time)
})
const scheduler = new Scheduler();
const addTask = (time, order) => {
    scheduler.add(() => timeout(time).then(() => console.log(order)))
}
addTask(400, 4)
addTask(200, 2)
addTask(400, 3)
addTask(100, 1)
