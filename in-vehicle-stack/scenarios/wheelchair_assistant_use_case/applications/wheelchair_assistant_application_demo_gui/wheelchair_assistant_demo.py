"""
This is a simple GUI that demonstrates the functionality
of the Wheelchair Assistant Application. It simulates the general workflow of the application as follows.

You start it by pushing START Button. The virtual car is now in a neutral stand.
By pushing the KEY Button you unlock the car and then with a Scale you can simulate
the distance measurement from the car to the handicap driver.
If the distance drops below 2 m the certain car equipment (door, steering wheel, seat) are being adjusted
to facilitate access to the car.
By pushing START THE CAR Button the equipment is being readjusted for the comfort driving.
"""


from tkinter import *


def switch_key():
    global lock_switch

    # Determine is on or off
    if not lock_switch:
        my_label.config(text="The Car is unlocked", fg="green")
        Key_Button.config(text='KEY')
        lock_switch = True
    else:
        my_label.config(text="The Car is locked", fg="grey")
        Key_Button.config(text='KEY')
        lock_switch = False


def switch_car():
    global lock_switch, car_on, dist_value
    if not car_on:
        if lock_switch and int(dist_value.get()) < 1:
            switch_key()
            seat_label.config(text='Seat: readjusted', fg='blue')
            door_label.config(text='Door: closed', fg='blue')
            steering_wheel_label.config(text='Steering Wheel: readjusted', fg='blue')
            Start_Button.config(text='STOP THE CAR')
            car_on = True
    else:
        switch_key()
        seat_label.config(text='Seat: adjusted', fg='green')
        door_label.config(text='Door: opened', fg='green')
        steering_wheel_label.config(text='Steering Wheel: adjusted', fg='green')
        Start_Button.config(text='START THE CAR')
        car_on = False


def switch_distance(val):
    global lock_switch

    if lock_switch and int(val) <= 2:
        seat_label.config(text='Seat: adjusted', fg='green')
        door_label.config(text='Door: opened', fg='green')
        steering_wheel_label.config(text='Steering Wheel: adjusted', fg='green')
    else:
        seat_label.config(text='Seat: neutral', fg='grey')
        door_label.config(text='Door: closed', fg='grey')
        steering_wheel_label.config(text='Steering Wheel: neutral', fg='grey')


lock_switch = True
car_on = False

top = Tk()
top.geometry("1000x600")
# top.config(background='lightblue')

my_label = Label(top, fg="grey", font=("Helvetica", 20))
my_label.place(x=100, y=50)

seat_label = Label(top,text='Seat', fg='grey', font=('Helvetica', 20))
seat_label.place(x=600, y=50)

door_label = Label(top,text='Door', fg='grey', font=('Helvetica', 20))
door_label.place(x=600, y=100)

steering_wheel_label = Label(top,text='Steering_Wheel', fg='grey', font=('Helvetica', 20))
steering_wheel_label.place(x=600, y=150)

dist_value = IntVar()
distance = Scale(top, orient='horizontal', from_=10, to=0, resolution=2, length=200,
                 label='Distance to car (m)', command=switch_distance, font=('Helvetica', 15), variable=dist_value)
distance.set(500)
distance.place(x=100, y=200)

Key_Button = Button(top, text ="START", width=15, height=4, font=("Helvetica", 10), command=switch_key)
Key_Button.place(x=150, y=100)

Start_Button = Button(top, text ="START THE CAR", width=15, height=4, font=("Helvetica", 10), command=switch_car)
Start_Button.place(x=150, y=400)
top.mainloop()
